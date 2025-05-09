use coin_cbc::{Col, Model, Sense};
use egg::*;
use std::collections::{HashMap, HashSet};
use crate::veclang::{VecLang};

pub trait LpCostFunction<L: Language> {
    fn node_cost(&mut self, enode: &L) -> f64;
}

impl LpCostFunction<VecLang> for AstSize {
    fn node_cost(&mut self, enode: &VecLang) -> f64 {
        const LITERAL: f64 = 0.0;
        const STRUCTURE: f64 = 2000.0;
        const VEC_OP: f64 = 1.0;
        const OP: f64 = 1.0;

        let op_cost = match enode {
            // You get literals for extremely cheap
            // VecLang::Num(value) => (*value * 4_000) as usize,
            VecLang::Num(..) => LITERAL,
            VecLang::Symbol(..) => LITERAL,

            // Vectors are cheap if they have literal values
            VecLang::Vec(..) => STRUCTURE,

            // But scalar and vector ops cost something
            VecLang::Add(..) => OP * 10_000.0,
            VecLang::Mul(..) => OP * 10_000.0,
            VecLang::Minus(..) => OP * 10_000.0,
            VecLang::Neg(..) => 10_000.0 * OP,
            
            VecLang::Rot(..) => VEC_OP * 50.0,
            VecLang::VecAdd(..) => VEC_OP,
            VecLang::VecMinus(..) => VEC_OP,
            VecLang::VecMul(..) => VEC_OP * 100.0,

            VecLang::VecAddRotF(..) | VecLang::VecAddRotS(..) => VEC_OP * 1051.0,
            VecLang::VecMinusRotF(..) | VecLang::VecMinusRotS(..) => VEC_OP * 1051.0,
            VecLang::VecMulRotF(..) | VecLang::VecMulRotS(..) => VEC_OP * 2150.0,

            VecLang::VecAddRotP(..) => VEC_OP * 5_000.0,
            VecLang::VecMinusRotP(..) => VEC_OP * 5_000.0,
            VecLang::VecMulRotP(..) => VEC_OP * 7_000.0,

            VecLang::VecNeg(..) => VEC_OP,
        };

        op_cost as f64
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "lp")))]
pub struct LpExtractor<'a, L: Language, N: Analysis<L>> {
    egraph: &'a EGraph<L, N>,
    model: Model,
    vars: HashMap<Id, ClassVars>,
}

struct ClassVars {
    active: Col,
    order: Col,
    nodes: Vec<Col>,
}

impl<'a, L, N> LpExtractor<'a, L, N>
where
    L: Language,
    N: Analysis<L>,
{
    /// Create an [`LpExtractor`] using costs from the given [`LpCostFunction`].
    /// See those docs for details.
    pub fn new<CF>(egraph: &'a EGraph<L, N>, mut cost_function: CF) -> Self
    where
        CF: LpCostFunction<L>,
    {
        let max_order = egraph.total_number_of_nodes() as f64 * 10.0;

        let mut model = Model::default();

        let vars: HashMap<Id, ClassVars> = egraph
            .classes()
            .map(|class| {
                let cvars = ClassVars {
                    active: model.add_binary(),
                    order: model.add_col(),
                    nodes: class.nodes.iter().map(|_| model.add_binary()).collect(),
                };
                model.set_col_upper(cvars.order, max_order);
                (class.id, cvars)
            })
            .collect();

        let mut cycles: HashSet<(Id, usize)> = Default::default();
        find_cycles(egraph, |id, i| {
            cycles.insert((id, i));
        });

        for (&id, class) in &vars {
            // class active == some node active
            // sum(for node_active in class) == class_active
            let row = model.add_row();
            model.set_row_equal(row, 0.0);
            model.set_weight(row, class.active, -1.0);
            for &node_active in &class.nodes {
                model.set_weight(row, node_active, 1.0);
            }

            for (i, (node, &node_active)) in egraph[id].iter().zip(&class.nodes).enumerate() {
                if cycles.contains(&(id, i)) {
                    model.set_col_upper(node_active, 0.0);
                    model.set_col_lower(node_active, 0.0);
                    continue;
                }

                for child in node.children() {
                    let child_active = vars[child].active;
                    // node active implies child active, encoded as:
                    //   node_active <= child_active
                    //   node_active - child_active <= 0
                    let row = model.add_row();
                    model.set_row_upper(row, 0.0);
                    model.set_weight(row, node_active, 1.0);
                    model.set_weight(row, child_active, -1.0);
                }
            }
        }

        model.set_obj_sense(Sense::Minimize);
        for class in egraph.classes() {
            for (node, &node_active) in class.iter().zip(&vars[&class.id].nodes) {
                model.set_obj_coeff(node_active, cost_function.node_cost(node));
            }
        }

        dbg!(max_order);

        Self {
            egraph,
            model,
            vars,
        }
    }

    /// Set the cbc timeout in seconds.
    pub fn timeout(&mut self, seconds: f64) -> &mut Self {
        self.model.set_parameter("seconds", &seconds.to_string());
        self
    }

    /// Extract a single rooted term.
    ///
    /// This is just a shortcut for [`LpExtractor::solve_multiple`].
    pub fn solve(&mut self, root: Id) -> RecExpr<L> {
        self.solve_multiple(&[root]).0
    }

    /// Extract (potentially multiple) roots
    pub fn solve_multiple(&mut self, roots: &[Id]) -> (RecExpr<L>, Vec<Id>) {
        let egraph = self.egraph;

        for class in self.vars.values() {
            self.model.set_binary(class.active);
        }

        for root in roots {
            let root = &egraph.find(*root);
            self.model.set_col_lower(self.vars[root].active, 1.0);
        }

        let solution = self.model.solve();
        // log::info!(
        //     "CBC status {:?}, {:?}",
        //     solution.raw().status(),
        //     solution.raw().secondary_status()
        // );

        let mut todo: Vec<Id> = roots.iter().map(|id| self.egraph.find(*id)).collect();
        let mut expr = RecExpr::default();
        // converts e-class ids to e-node ids
        let mut ids: HashMap<Id, Id> = HashMap::default();

        while let Some(&id) = todo.last() {
            if ids.contains_key(&id) {
                todo.pop();
                continue;
            }
            let v = &self.vars[&id];
            assert!(solution.col(v.active) > 0.0);
            let node_idx = v.nodes.iter().position(|&n| solution.col(n) > 0.0).unwrap();
            let node = &self.egraph[id].nodes[node_idx];
            if node.all(|child| ids.contains_key(&child)) {
                let new_id = expr.add(node.clone().map_children(|i| ids[&self.egraph.find(i)]));
                ids.insert(id, new_id);
                todo.pop();
            } else {
                todo.extend_from_slice(node.children())
            }
        }

        let root_idxs = roots.iter().map(|root| ids[root]).collect();

        // assert!(expr.is_dag(), "LpExtract found a cyclic term!: {:?}", expr);
        // eprintln!("LpExtract found a cyclic term!: {:?}", expr);
        (expr, root_idxs)
    }
}

fn find_cycles<L, N>(egraph: &EGraph<L, N>, mut f: impl FnMut(Id, usize))
where
    L: Language,
    N: Analysis<L>,
{
    enum Color {
        White,
        Gray,
        Black,
    }
    type Enter = bool;

    let mut color: HashMap<Id, Color> = egraph.classes().map(|c| (c.id, Color::White)).collect();
    let mut stack: Vec<(Enter, Id)> = egraph.classes().map(|c| (true, c.id)).collect();

    while let Some((enter, id)) = stack.pop() {
        if enter {
            *color.get_mut(&id).unwrap() = Color::Gray;
            stack.push((false, id));
            for (i, node) in egraph[id].iter().enumerate() {
                for child in node.children() {
                    match &color[child] {
                        Color::White => stack.push((true, *child)),
                        Color::Gray => f(id, i),
                        Color::Black => (),
                    }
                }
            }
        } else {
            *color.get_mut(&id).unwrap() = Color::Black;
        }
    }
}
