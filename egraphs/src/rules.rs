use std::usize;
// use egg::{RecExpr, Language, Analysis, EClass, EGraph, Id, Pattern, Subst, Searcher};
use egg::*;
use crate::{
    extractor::Extractor,
    veclang::{ConstantFold, Egraph, VecLang},
    runner::Runner,
    cost::VecCostFn,
    // rewrite::Rewrite
};
// use crate::rewrite::ConditionalApplier;
use std::collections::HashMap;
use std::collections::HashSet;
use log::debug;
// Check if all the variables, in this case memories, are equivalent

/// Run the rewrite rules over the input program and return the best (cost, program)
use std::time::Instant;

pub fn run(
    prog: &RecExpr<VecLang>,
    timeout: u64,
    vector_width: usize,
    rule_filtering: bool,
    _sorting: bool,
    exp_rules: bool,
) -> (f64, RecExpr<VecLang>) {

    let optimized_rw = false;
    let sorting = true;

    let mut initial_operations = Vec::new();
    eprintln!("starting vectorization ...");
    eprintln!("vector width is {:?}", vector_width);
    eprintln!("rule_filtering is {:?}", rule_filtering);
    eprintln!("sorting is {:?}", sorting);
    eprintln!("exp_rules is {:?}", exp_rules);

    debug!("the begining expre is : {:?}", prog);

    for node in prog.as_ref() {
        debug!("node is : {:?}", node);
        match node {
              VecLang::Num(_) 
            | VecLang::Symbol(_) 
            | VecLang::Vec(_) 
            | VecLang::VecNeg(_) 
            |  VecLang::Rot(_) 
            => continue,

            VecLang::Add(_) | VecLang::VecAdd(_)
            | VecLang::VecAddRotF(_) | VecLang::VecAddRotP(_) 
            => initial_operations.push("Add".to_string()),

            VecLang::Minus(_) | VecLang::VecMinus(_)
            | VecLang::VecMinusRotF(_) | VecLang::VecMinusRotP(_) 
            => initial_operations.push("Min".to_string()),

            VecLang::Mul(_) | VecLang::VecMul(_) 
            | VecLang::VecMulRotF(_) | VecLang::VecMulRotP(_) 
            => initial_operations.push("Mul".to_string()),

            VecLang::Neg(_) => initial_operations.push("Neg".to_string())
        }
    }

    let mut rules_info : HashMap<String, Vec<String>> = HashMap::new();
    let mut initial_rules : Vec<Rewrite<VecLang, ConstantFold>> = Vec::new();
    let mut rules : Vec<Rewrite<VecLang, ConstantFold>> = Vec::new();
    // Initialize the rule set based on the vector width
    generate_rules(
        vector_width,
        optimized_rw,
        exp_rules, 
        initial_operations.clone(),
        &mut rules_info,
        &mut initial_rules,
        &mut rules
    );

    // generate_rules_unstructured_code(
    //     optimized_rw,
    //     initial_operations.clone(),
    //     &mut rules_info,
    //     &mut initial_rules,
    //     &mut rules
    // );

    // generate_associativity_and_commutativity_rules(
    //     optimized_rw,
    //     initial_operations,
    //     &mut rules_info,
    //     &mut initial_rules,
    //     &mut rules
    // );
    

    for rw in initial_rules.iter() {
        debug!("initial rules are: {:?}", rw.name.as_str());

    }

    //let mut stop_flag = false;
    // let start = "(Vec 
    //     (+ (+ (* a0 (* b0 c0)) d0) (+ (* a1 (* b1 c1)) (* a3 (* b3 c3))))
    //     (+ (* a2 (* b2 c2)) d1)
    //     (+ d2 (* a4 (* b4 c4)))
    //     )".parse().unwrap();

    // let start = "(Vec 
    // a
    // c
    // )".parse().unwrap();


    // let start = "(
    //     + (+ (+ a b) (+ c d)) (* e f)
    // )".parse().unwrap();


    // Start timing the e-graph building process
    let start_time = Instant::now();

    // Initialize the e-graph with constant folding enabled and add a zero literal
    let mut init_eg = Egraph::new(ConstantFold);
    init_eg.add(VecLang::Num(0));

    type MyRunner = Runner<VecLang, ConstantFold>;


    let runner = MyRunner::new(Default::default())
        .with_egraph(init_eg)
        .with_expr(&prog)
        .with_node_limit(2_000_000)
        .with_time_limit(std::time::Duration::from_secs(timeout))
        .with_iter_limit(10_000)
        // .run(&initial_rules);
        .run(&rules, &initial_rules, rules_info, optimized_rw);

        let report = runner.report();
        eprintln!("report : {:?}", report);
        /* for the rules , if the rule is expensive we add the prefix exp to its name */


    // Stop timing after the e-graph is built
    let build_time = start_time.elapsed();
    eprintln!("E-graph built in {:?}", build_time);

    // Print the reason for stopping to STDERR
    eprintln!(
        "Stopped after {} iterations, reason: {:?}",
        runner.iterations.len(),
        runner.stop_reason
    );

   

    // Extract the e-graph and the root node
    let (eg, root) = (runner.egraph, runner.roots[0]);
    eprintln!("final number of enodes : {:?}", eg.total_size());
    // print_egraph(eg.clone());


    let best_cost: f64;
    let best_expr; // best_expr: RecExpr<VecLang> = RecExpr::default();
    eprintln!("begining of extraction 0 .... ");

    /* we have 3 ways fot the extraction:
        1) greedy_extraction: takes decisions locally
        2) exhaustive_extraction: exploring all possibilities
        3) sa_extraction: based on simulating annealing metaheuristic
    */

   
    /************************************ greedy extraction ******************************************/
    let start_extract_time = Instant::now();
    let mut extractor = Extractor::new(&eg, VecCostFn { egraph: &eg }, root, true);
    (best_cost, best_expr) = extractor.find_best(root);
    let extract_time = start_extract_time.elapsed();

    /********************************** Exhaustive extraction ***************************************/
    // let start_extract_time = Instant::now();
    // let mut extractor = Extractor::new(&eg);
    // extractor.find_best(
    //     vec![root],
    //     HashMap::new(),
    //     root,
    //     0,
    //     0,
    //     vec![],
    //     &mut best_cost,
    //     &mut best_expr,
    //     &mut HashMap::new(), 
    // );
    // let extract_time = start_extract_time.elapsed();

    /************************************ SA extraction *************************************************/
    // let start_extract_time = Instant::now();
    // let mut extractor = Extractor::new(&eg);
    // let mut n_cost:usize = 0;

    // // Parameters for simulated annealing
    // let max_iteration = 9999999;
    // let initial_temp = 99990.0;
    // let cooling_rate = 0.99;

    // (best_cost, best_expr) = extractor.find_best(
    //     &eg,
    //     root,
    //     max_iteration,
    //     initial_temp,
    //     cooling_rate,
    // );

    // let extract_time = start_extract_time.elapsed();

    // Stop timing after the extraction is complete
    eprintln!("display final results");
    eprintln!("Expression extraction took {:?}", extract_time);
    eprintln!("Final cost is {}", best_cost);
    eprintln!("Extracted Expression : {}", best_expr);

    // Return the extracted cost and expression
    (best_cost, best_expr)

}


pub fn print_egraph(
    egraph: Egraph
)
{

    eprintln!("***************egraph******************");

                for eclass in egraph.classes() {
                    // Print the e-class ID
                    eprint!("E-Class {{Id: {}}} =", eclass.id);
            
                    // Iterate over all enodes in the e-class and print them
                    for enode in &eclass.nodes {
                        eprint!(" {:?}", enode);
                    }
            
                    // Newline after each e-class
                    eprintln!();
                }
    // Create a map to hold the connections for each eclass
    let mut connections: HashMap<Id, HashSet<Id>> = HashMap::new();

    for class in egraph.classes() {
        let class_id = class.id;    

        // Initialize the set of connections if not already present
        let mut class_connections = HashSet::new();
        // eprintln!("The size of this eclass with id {:?} is : {:?}", class_id, class.len());

        for (_node_index, node) in class.iter().enumerate() {
            // Print the content of each enode
            // eprintln!("  Enode {}: {:?}", node_index + 1, node);
            for child in node.children() {
                // eprintln!("    Child: {:?}", child);
                // Add child to the list of connections
                class_connections.insert(*child);
            }
        }

        connections.insert(class_id, class_connections);
    }

    // Print the graph in the terminal
    for (class_id, class_connections) in &connections {
        let connections_str: String = class_connections
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        eprintln!("Class {} linked to {}", class_id, connections_str);
    }
}


    pub fn vectorization_rules(
        vector_width: usize,
        optimized_rw: bool,
        initial_operations: Vec<String>,
        rules_info: &mut HashMap<String, Vec<String>>,
        initial_rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
        rules: &mut Vec<Rewrite<VecLang, ConstantFold>>
    ) {

        let mut searcher_add = Vec::new();
        let mut searcher_mul = Vec::new();
        let mut searcher_sub = Vec::new();
        let mut searcher_neg = Vec::new();

        let mut applier_1 = Vec::new();
        let mut applier_2 = Vec::new();

        for i in 0..vector_width {
            searcher_add.push(format!("( + ?a{} ?b{}) ", i, i));
            searcher_mul.push(format!("( * ?a{} ?b{}) ", i, i));
            searcher_sub.push(format!("( - ?a{} ?b{}) ", i, i));
            searcher_neg.push(format!("( - ?a{}) ", i));

            applier_1.push(format!("?a{} ", i));
            applier_2.push(format!("?b{} ", i));
        }

        let lhs_add: Pattern<VecLang> = format!("(Vec {})", searcher_add.concat()).parse().unwrap();
        let lhs_mul: Pattern<VecLang> = format!("(Vec {})", searcher_mul.concat()).parse().unwrap();
        let lhs_sub: Pattern<VecLang> = format!("(Vec {})", searcher_sub.concat()).parse().unwrap();
        let lhs_neg: Pattern<VecLang> = format!("(Vec {})", searcher_neg.concat()).parse().unwrap();

        // Parse the right-hand side patterns
        let rhs_add: Pattern<VecLang> = format!(
            "(VecAdd (Vec {}) (Vec {}))",
            applier_1.concat(),
            applier_2.concat()
        )
        .parse()
        .unwrap();
        debug!("{} => {}", lhs_add, rhs_add);
        let rhs_mul: Pattern<VecLang> = format!(
            "(VecMul (Vec {}) (Vec {}))",
            applier_1.concat(),
            applier_2.concat()
        )
        .parse()
        .unwrap();

        let rhs_sub: Pattern<VecLang> = format!(
            "(VecMinus (Vec {}) (Vec {}))",
            applier_1.concat(),
            applier_2.concat()
        )
        .parse()
        .unwrap();

        let rhs_neg: Pattern<VecLang> = format!("(VecNeg (Vec {}) )", applier_1.concat(),)
            .parse()
            .unwrap();

        if optimized_rw {
            let rule_name_add = format!("add-vectorize");
            let rule_name_mul = format!("mul-vectorize");
            let rule_name_sub = format!("sub-vectorize");
            let rule_name_neg = format!("neg_vectorize");
            // Conditionally push the rewrite rules into the rules vector
            if initial_operations.contains(&"Add".to_string()) {
                initial_rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
            }

            if initial_operations.contains(&"Mul".to_string()) {
                initial_rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
            }

            if initial_operations.contains(&"Min".to_string()) {
                initial_rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
            }

            if initial_operations.contains(&"Neg".to_string()) {
                initial_rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
            }

            rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
            rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
            rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
            rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
            rules_info.insert(rule_name_add.clone(), vec![lhs_add.clone().to_string(), rhs_add.clone().to_string()]);
            rules_info.insert(rule_name_mul.clone(), vec![lhs_mul.clone().to_string(), rhs_mul.clone().to_string()]);
            rules_info.insert(rule_name_sub.clone(),  vec![lhs_sub.clone().to_string(), rhs_sub.clone().to_string()]);
            rules_info.insert(rule_name_neg.clone(), vec![lhs_neg.clone().to_string(), rhs_neg.clone().to_string()]);
        } else {
            let rule_name_add = format!("add-vectorize");
            let rule_name_mul = format!("mul-vectorize");
            let rule_name_sub = format!("sub-vectorize");
            let rule_name_neg = format!("neg_vectorize");
            // Push all rewrite rules into the rules vector
            initial_rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
            initial_rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
            initial_rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
            initial_rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
            rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
            rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
            rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
            rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
            rules_info.insert(rule_name_add, vec![lhs_add.clone().to_string(), rhs_add.clone().to_string()]);
            rules_info.insert(rule_name_mul, vec![lhs_mul.clone().to_string(), rhs_mul.clone().to_string()]);
            rules_info.insert(rule_name_sub,  vec![lhs_sub.clone().to_string(), rhs_sub.clone().to_string()]);
            rules_info.insert(rule_name_neg, vec![lhs_neg.clone().to_string(), rhs_neg.clone().to_string()]);
        }

    }

    pub fn is_not_vector_of_scalar_operations(
        vars: &'static str, // Make vars static
    ) -> impl Fn(&mut Egraph, Id, &Subst) -> bool + 'static {
        let vars = &vars[5..vars.len() - 2];
        let vars_vector = vars.split(" ").collect::<Vec<&str>>();
        move |egraph, _, subst| {
            let mut no_scalar_operations = true;
            for var in &vars_vector {
                let var = var.parse().unwrap();
                no_scalar_operations = no_scalar_operations
                    && egraph[subst[var]].nodes.iter().any(|n| match n {
                        VecLang::Num(..) | VecLang::Symbol(..) => true,
                        _ => false,
                    });
                if !no_scalar_operations {
                    break;
                }
            }
            return no_scalar_operations;
        }
    }

    pub fn rotation_rules(vector_width: usize) -> Vec<Rewrite<VecLang, ConstantFold>> {
        // Modify the function to take a static string

        let mut rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![];

        let vector_width: usize = vector_width;

        // Create `lhs` as a static str directly
        let lhs = Box::leak(
            format!(
                "(Vec {})",
                (0..vector_width)
                    .map(|i| format!("?a{} ", i))
                    .collect::<String>()
            )
            .into_boxed_str(),
        ); // Convert String to &'static str using Box::leak

        let searcher: Pattern<VecLang> = lhs.parse().unwrap();

        for i in 1..vector_width {
            let rhs = format!(
                "(<< (Vec {}) {})",
                (0..vector_width)
                    .map(|j| format!("?a{} ", (i + j) % vector_width))
                    .collect::<String>(),
                vector_width - i
            );
            let applier: Pattern<VecLang> = rhs.parse().unwrap();

            // Pass `lhs` as a &'static str, no need for clone
            let rule: Vec<Rewrite<VecLang, ConstantFold>> = rewrite!(format!("rotations-{}", i); { searcher.clone() } <=> { applier.clone() } if is_not_vector_of_scalar_operations(lhs));

            rules.extend(rule);
        }

        rules
    }


    pub fn split_vectors(
        vector_width: usize,
        initial_operations: Vec<String>
    ) -> Vec<Rewrite<VecLang, ConstantFold>> {
        let mut rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![];

        // Store vector width in a constant

        /************************** Zakaria implementaion *********************************/

        let lhs = format!(
            "(Vec {})",
            (0..vector_width)
                .map(|i| format!("?a{} ", i))
                .collect::<String>()
        );

        let searcher: Pattern<VecLang> = lhs.parse().unwrap();

        for i in 0..vector_width {
            let vector1_add = format!(
                "(Vec {})",
                (0..vector_width)
                    .map(|j| if i == j {
                        "0 ".to_string()
                    } else {
                        format!("?a{} ", j)
                    })
                    .collect::<String>()
            );
            let vector1_mul = format!(
                "(Vec {})",
                (0..vector_width)
                    .map(|j| if i == j {
                        "1 ".to_string()
                    } else {
                        format!("?a{} ", j)
                    })
                    .collect::<String>()
            );

            let vector2_add = format!(
                "(Vec {})",
                (0..vector_width)
                    .map(|j| if i == j {
                        format!("?a{} ", j)
                    } else {
                        "0 ".to_string()
                    })
                    .collect::<String>()
            );
            let vector2_mul = format!(
                "(Vec {})",
                (0..vector_width)
                    .map(|j| if i == j {
                        format!("?a{} ", j)
                    } else {
                        "1 ".to_string()
                    })
                    .collect::<String>()
            );

            let rhs_add = format!("(VecAdd {} {})", vector1_add, vector2_add);
            let rhs_mul = format!("(VecMul {} {})", vector1_mul, vector2_mul);
            let applier_add: Pattern<VecLang> = rhs_add.parse().unwrap();
            let applier_mul: Pattern<VecLang> = rhs_mul.parse().unwrap();

            if initial_operations.contains(&"Add".to_string()) {
                rules.push(rewrite!(format!("exp-split-add-{}", i); {  searcher.clone()} => {  applier_add}));
            }

            if initial_operations.contains(&"Mul".to_string()) {
                rules.push(rewrite!(format!("exp-split-mul-{}", i); {  searcher.clone()} => {  applier_mul}))
            }
        }

        rules
    }

    pub fn commutativity_rules(vector_width: usize) -> Vec<Rewrite<VecLang, ConstantFold>> {
        let mut rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![];

        for i in (0..vector_width).step_by(2) {
            // Create the lhs and rhs expressions directly as strings
            let lhs = format!("(+ (* a{} b{}) c{})", i, i, i);
            let rhs = format!("(+ c{} (* a{} b{}))", i, i, i);

            // Parse the expressions into patterns
            let lhs_pattern: Pattern<VecLang> = lhs.parse().unwrap();
            let rhs_pattern: Pattern<VecLang> = rhs.parse().unwrap();

            // Add the rewrite rule using a literal string for the rule name
            rules.push(rewrite!(format!("exp-assoc-{}", i); lhs_pattern => rhs_pattern));
        }

        rules
    }


    pub fn operations_rules(
        vector_width: usize, 
        optimized_rw: bool,
        initial_operations: Vec<String>,
        rules_info: &mut HashMap<String, Vec<String>>,
        initial_rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
        rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
    ) {


        // Iterate over each possible position in the vector
        for i in 0..vector_width {
            // Initialize vectors to store different patterns
            let mut vector_add = Vec::new();
            let mut vector_mul = Vec::new();
            let mut vector_sub = Vec::new();
            let mut vector_neg = Vec::new();
            let mut vector1 = Vec::new();
            let mut vector2 = Vec::new();
            let mut vector1_neg = Vec::new();
            let mut vector2_neg = Vec::new();
            let mut vector2_mul = Vec::new();

            // Iterate over each element in the vector
            for j in 0..vector_width {
                if i == j {
                    // When i equals j, insert the operations
                    vector_add.push(format!("( + ?a{}1 ?a{}2) ", j, j));
                    vector_mul.push(format!("( * ?a{}1 ?a{}2) ", j, j));
                    vector_sub.push(format!("( - ?a{}1 ?a{}2) ", j, j));
                    vector_neg.push(format!("( - ?a{}) ", j));
                    vector1_neg.push("0 ".to_string());
                    vector2_neg.push(format!("?a{}  ", j));
                    vector1.push(format!("?a{}1 ", j));
                    vector2.push(format!("?a{}2 ", j));
                    vector2_mul.push(format!("?a{}2 ", j));
                } else {
                    // When i does not equal j, insert the vector elements
                    vector_add.push(format!("?a{} ", j));
                    vector_mul.push(format!("?a{} ", j));
                    vector_sub.push(format!("?a{} ", j));
                    vector_neg.push(format!("?a{} ", j));
                    vector1.push(format!("?a{} ", j));
                    vector1_neg.push(format!("?a{} ", j));
                    vector2_neg.push("0 ".to_string());
                    vector2_mul.push("1 ".to_string());
                    vector2.push("0 ".to_string());
                }
            }

            // Parse the left-hand side patterns
            let lhs_add: Pattern<VecLang> = format!("(Vec {})", vector_add.concat()).parse().unwrap();
            let lhs_mul: Pattern<VecLang> = format!("(Vec {})", vector_mul.concat()).parse().unwrap();
            let lhs_sub: Pattern<VecLang> = format!("(Vec {})", vector_sub.concat()).parse().unwrap();
            let lhs_neg: Pattern<VecLang> = format!("(Vec {})", vector_neg.concat()).parse().unwrap();

            // Parse the right-hand side patterns
            let rhs_add: Pattern<VecLang> = format!(
                "(VecAdd (Vec {}) (Vec {}))",
                vector1.concat(),
                vector2.concat()
            )
            .parse()
            .unwrap();

            let rhs_mul: Pattern<VecLang> = format!(
                "(VecMul (Vec {}) (Vec {}))",
                vector1.concat(),
                vector2_mul.concat()
            )
            .parse()
            .unwrap();

            let rhs_sub: Pattern<VecLang> = format!(
                "(VecMinus (Vec {}) (Vec {}))",
                vector1.concat(),
                vector2.concat()
            )
            .parse()
            .unwrap();

            let rhs_neg: Pattern<VecLang> = format!(
                "(VecMinus (Vec {}) (Vec {}))",
                vector1_neg.concat(),
                vector2_neg.concat()
            )
            .parse()
            .unwrap();

            if optimized_rw {
                let rule_name_add = format!("add-split-{}", i);
                let rule_name_mul = format!("mul-split-{}", i);
                let rule_name_sub = format!("sub-split-{}", i);
                let rule_name_neg = format!("neg-split-{}", i);

                // Condtionally push the rewrite rules into the rules vector
                if initial_operations.contains(&"Add".to_string()) {
                    initial_rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
                }

                if initial_operations.contains(&"Mul".to_string()) {
                    initial_rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
                }

                if initial_operations.contains(&"Min".to_string()) {
                    initial_rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
                }

                if initial_operations.contains(&"Neg".to_string()) {
                    initial_rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
                }

                rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
                rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
                rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
                rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
                rules_info.insert(rule_name_add, vec!["Add".to_string(),"Add".to_string()]);
                rules_info.insert(rule_name_mul, vec!["Mul".to_string(), "Mul".to_string()]);
                rules_info.insert(rule_name_sub,  vec!["Minus".to_string(), "Minus".to_string()]);
                rules_info.insert(rule_name_neg, vec!["Neg".to_string(), "Neg".to_string()]);

            } 
            else
            {
                let rule_name_add = format!("add-split-{}", i);
                let rule_name_mul = format!("mul-split-{}", i);
                let rule_name_sub = format!("sub-split-{}", i);
                let rule_name_neg = format!("neg-split-{}", i);
                // Push all rewrite rules into the rules vector
                initial_rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
                initial_rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
                initial_rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
                initial_rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
                rules.push(rewrite!(rule_name_add.clone(); { lhs_add.clone() } => { rhs_add.clone() }));
                rules.push(rewrite!(rule_name_mul.clone(); { lhs_mul.clone() } => { rhs_mul.clone() }));
                rules.push(rewrite!(rule_name_sub.clone(); { lhs_sub.clone() } => { rhs_sub.clone() }));
                rules.push(rewrite!(rule_name_neg.clone(); { lhs_neg.clone() } => { rhs_neg.clone() }));
                rules_info.insert(rule_name_add, vec!["Add".to_string(), "Add".to_string()]);
                rules_info.insert(rule_name_mul, vec!["Mul".to_string(), "Mul".to_string()]);
                rules_info.insert(rule_name_sub,  vec!["Minus".to_string(), "Minus".to_string()]);
                rules_info.insert(rule_name_neg, vec!["Neg".to_string(), "Neg".to_string()]);
            }   
        }

    }


    pub fn generate_rules(
        vector_width: usize,
        optimized_rw: bool,
        exp_rules: bool,
        initial_operations: Vec<String>,
        rules_info: &mut HashMap<String, Vec<String>>,
        initial_rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
        rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
    ) {

        let constant_folding_rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
            // rewrite!("con-1"; "0" => "(+ 0 0)"),
            // rewrite!("con-2"; "1" => "(* 1 1)"),
            // rewrite!("add-0"; "(+ 0 ?a)" => "?a"),
            // rewrite!("add-0-2"; "(+ ?a 0)" => "?a"),
            // rewrite!("mul-0"; "(* 0 ?a)" => "0"),
            // rewrite!("mul-0-2"; "(* ?a 0)" => "0"),
            // rewrite!("mul-1"; "(* 1 ?a)" => "?a"),
            // rewrite!("mul-1-2"; "(* ?a 1)" => "?a"),
            // rewrite!("comm-factor-1"; "(+ (* ?a0 ?b0) (* ?a0 ?c0))" => "(* ?a0 (+ ?b0 ?c0))"),
            // rewrite!("comm-factor-2"; "(+ (* ?b0 ?a0) (* ?c0 ?a0))" => "(* ?a0 (+ ?b0 ?c0))"),
        ];



        rules.extend(constant_folding_rules);

        let mut rules_info_med : HashMap<String, Vec<String>> = HashMap::new();
        let mut rules_med : Vec<Rewrite<VecLang, ConstantFold>> = Vec::new();
        let mut initial_rules_med : Vec<Rewrite<VecLang, ConstantFold>> = Vec::new();

        /************************************* rule generation **********************************/
        operations_rules(
            vector_width,
            optimized_rw,
            initial_operations.clone(),
            &mut rules_info_med,
            &mut initial_rules_med,
            &mut rules_med
        );

        rules_info.extend(rules_info_med.clone());
        initial_rules.extend(initial_rules_med.clone());
        rules.extend(rules_med.clone());

        rules_info_med.clear();
        rules_med.clear();
        initial_rules_med.clear();

        vectorization_rules(
            vector_width,
            optimized_rw,
            initial_operations.clone(),
            &mut rules_info_med,
            &mut initial_rules_med,
            &mut rules_med
        );

        debug!("vectorization rules selected : {:?}", initial_rules_med);
        rules_info.extend(rules_info_med.clone());
        initial_rules.extend(initial_rules_med.clone());
        rules.extend(rules_med.clone());

       
        rules_info_med.clear();
        rules_med.clear();
        initial_rules_med.clear();

        // balancing_rules(
        //     optimized_rw,
        //     initial_operations.clone(),
        //     &mut rules_info_med,
        //     &mut initial_rules_med,
        //     &mut rules_med
        // );


        // debug!("balancing rules selected : {:?}", initial_rules_med);
        // rules_info.extend(rules_info_med);
        // initial_rules.extend(initial_rules_med);
        // rules.extend(rules_med);
        // eprintln!("===============================================================");

        // for rw in initial_rules {
        //     debug!("initial rules : {:?}", rw);
        // }

        /************************************* creating rule set **********************************/
        if exp_rules {
            let rotation_rules = rotation_rules(vector_width);
            let split_vectors = split_vectors(vector_width, initial_operations.clone());
            rules.extend(rotation_rules);
            rules.extend(split_vectors);
        }

    }

   
    // pub fn generate_rules_unstructured_code(
    //     _optimized_rw: bool,
    //     _initial_operations: Vec<String>,
    //     _rules_info: &mut HashMap<String, Vec<String>>,
    //     _initial_rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
    //     rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
    // ) {

    //     let max_num_elements = 10;
    //     let elements_as_str : Vec<String> = Vec::new();
       
    //     // generate all possible left hand sides
    //     for num_ele_index in 3..=max_num_elements {
    //         let mut lhs_pattern_add = Vec::new();
    //         let mut lhs_pattern_sub = Vec::new();
    //         let mut lhs_pattern_mul = Vec::new();

    //         // Generate the LHS pattern
    //         for num_elements in 0..(num_ele_index - 1) {
    //             lhs_pattern_add.push(format!("(+ ?a{} ", num_elements));
    //             lhs_pattern_sub.push(format!("(- ?a{} ", num_elements));
    //             lhs_pattern_mul.push(format!("(* ?a{} ", num_elements));
    //         }
    //         lhs_pattern_add.push(format!("?a{}", num_ele_index - 1));
    //         lhs_pattern_sub.push(format!("?a{}", num_ele_index - 1));
    //         lhs_pattern_mul.push(format!("?a{}", num_ele_index - 1));

    //         for _ in 0..(num_ele_index - 1) {
    //             lhs_pattern_add.push(')'.to_string());
    //             lhs_pattern_sub.push(')'.to_string());
    //             lhs_pattern_mul.push(')'.to_string());
    //         }

    //         let full_lhs_pattern_add_string : String = lhs_pattern_add.concat().parse().unwrap();
    //         let full_lhs_pattern_sub_string : String = lhs_pattern_sub.concat().parse().unwrap();
    //         let full_lhs_pattern_mul_string : String = lhs_pattern_mul.concat().parse().unwrap();

    //         eprintln!("lhs for add is {:?}", full_lhs_pattern_add_string);
    //         debug!("lhs for sub is {:?}", full_lhs_pattern_sub_string);
    //         debug!("lhs for mul is {:?}", full_lhs_pattern_mul_string);

    //         let full_lhs_pattern_add : Pattern<VecLang> = lhs_pattern_add.concat().parse().unwrap();
    //         let full_lhs_pattern_sub : Pattern<VecLang> = lhs_pattern_sub.concat().parse().unwrap();
    //         let full_lhs_pattern_mul : Pattern<VecLang> = lhs_pattern_mul.concat().parse().unwrap();

    //         // Extract all `?a_i` terms from the LHS pattern
    //         let elements: Vec<String> = lhs_pattern_add // or lhs_pattern_sub or lhs_pattern_mul
    //         .iter()
    //         .filter_map(|s| {
    //             // Find the starting position of "?a" in the string
    //             s.find("?a").map(|start| s[start..].split_whitespace().next().unwrap().to_string())
    //         })
    //         .collect();
          

    //         let num_elements = elements.len();
    //         let half_elements = num_elements / 2;
    //         let modulo_results = num_elements % 2;

    //         let min_vect_width = if modulo_results == 0 {
    //             half_elements
    //         } else {
    //             half_elements + 1
    //         };

    //         let max_vect_width = num_elements - 1;
    //         let mut rhs_pattern_add = Vec::new();
    //         let mut rhs_pattern_sub = Vec::new();
    //         let mut rhs_pattern_mul = Vec::new();

    //         for vector_width_index in min_vect_width..=max_vect_width {
    //             debug!("in this iteration, vector_width_index is {:?}", vector_width_index);
    //             let mut cpt = 0; // Counter for the current vector position

    //         rhs_pattern_add.push("(Vec ".to_string());
    //         rhs_pattern_sub.push("(Vec ".to_string());
    //         rhs_pattern_mul.push("(Vec ".to_string());

    //         let mut total_rotations = 0;
    //         let mut is_first_vector = true;

    //         for (_index, element) in elements.iter().enumerate() {
    //             rhs_pattern_add.push(element.to_string());
    //             rhs_pattern_sub.push(element.to_string());
    //             rhs_pattern_mul.push(element.to_string());

    //             cpt += 1;

    //             if cpt != 1 && is_first_vector {       // there is no rotation for the first element and avoiding calculating the rotations of the second vectors
    //                 total_rotations +=1;
    //             }
    //             // Check if the current vector is full
    //             if cpt == vector_width_index {
    //                 rhs_pattern_add.push(") (Vec ".to_string());
    //                 rhs_pattern_sub.push(") (Vec ".to_string());
    //                 rhs_pattern_mul.push(") (Vec ".to_string());
    //                 cpt = 0;
    //                 is_first_vector = false;
    //             } else {
    //                 rhs_pattern_add.push(" ".to_string());
    //                 rhs_pattern_sub.push(" ".to_string());
    //                 rhs_pattern_mul.push(" ".to_string());
    //             }
    //         }

    //         let full_vectors = cpt == 0; // Determine whether the vectors are full or partial

    //         if cpt > 0 {
    //             for _ in cpt..vector_width_index {
    //                 rhs_pattern_add.push("0".to_string());
    //                 rhs_pattern_add.push(" ".to_string());
    //                 rhs_pattern_sub.push("0".to_string());
    //                 rhs_pattern_sub.push(" ".to_string());
    //                 rhs_pattern_mul.push("0".to_string());
    //                 rhs_pattern_mul.push(" ".to_string());
    //             }
    //             rhs_pattern_add.pop();
    //             rhs_pattern_add.push(")".to_string());
    //             rhs_pattern_sub.pop();
    //             rhs_pattern_sub.push(")".to_string());
    //             rhs_pattern_mul.pop();
    //             rhs_pattern_mul.push(")".to_string());
                
    //         } else {
    //             rhs_pattern_add.pop();
    //             rhs_pattern_add.push(")".to_string());
    //             rhs_pattern_sub.pop();
    //             rhs_pattern_sub.push(")".to_string());
    //             rhs_pattern_mul.pop();
    //             rhs_pattern_mul.push(")".to_string());
    //         }

    //         // Add VecAddRotF or VecAddRotP based on the vector completeness
    //         if full_vectors {
    //             rhs_pattern_add.insert(0, "(VecAddRotF ".to_string());
    //             rhs_pattern_sub.insert(0, "(VecMinusRotF ".to_string());
    //             rhs_pattern_mul.insert(0, "(VecMulRotF ".to_string());
    //         } else {
    //             rhs_pattern_add.insert(0, "(VecAddRotP ".to_string());
    //             rhs_pattern_sub.insert(0, "(VecMinusRotP ".to_string());
    //             rhs_pattern_mul.insert(0, "(VecMulRotP ".to_string());
    //         }

    //         // Add the total rotation count
    //         rhs_pattern_add.push(format!(" {})", total_rotations));
    //         rhs_pattern_sub.push(format!(" {})", total_rotations));
    //         rhs_pattern_mul.push(format!(" {})", total_rotations));

    //         let full_rhs_pattern_add_string: String = rhs_pattern_add.concat();
    //         let full_rhs_pattern_sub_string: String = rhs_pattern_sub.concat();
    //         let full_rhs_pattern_mul_string: String = rhs_pattern_mul.concat();

    //         eprintln!("rhs for add is {:?}", full_rhs_pattern_add_string);
    //         debug!("rhs for sub is {:?}", full_rhs_pattern_sub_string);
    //         debug!("rhs for mul is {:?}", full_rhs_pattern_mul_string);                

    //         let full_rhs_pattern_add : Pattern<VecLang> = rhs_pattern_add.concat().parse().unwrap();
    //         let full_rhs_pattern_sub : Pattern<VecLang> = rhs_pattern_sub.concat().parse().unwrap();
    //         let full_rhs_pattern_mul : Pattern<VecLang> = rhs_pattern_mul.concat().parse().unwrap();

    //         let rule_name_add = format!("add-vec-{}-{}", num_ele_index, vector_width_index);
    //         let rule_name_sub = format!("sub-vec-{}-{}", num_ele_index, vector_width_index);
    //         let rule_name_mul = format!("mul-vec-{}-{}", num_ele_index, vector_width_index);

    //         rules.push(rewrite!(
    //             rule_name_add.clone(); {full_lhs_pattern_add.clone()} => {full_rhs_pattern_add.clone()} if are_all_symbols_or_nums(elements.clone())
    //             )
    //         );
    //         rules.push(rewrite!(
    //             rule_name_sub.clone(); {full_lhs_pattern_sub.clone()} => {full_rhs_pattern_sub.clone()} if are_all_symbols_or_nums(elements.clone())
    //             )
    //         );
    //         rules.push(rewrite!(
    //             rule_name_mul.clone(); {full_lhs_pattern_mul.clone()} => {full_rhs_pattern_mul.clone()} if are_all_symbols_or_nums(elements.clone())
    //             )
    //         );

    //         rhs_pattern_add = Vec::new();
    //         rhs_pattern_sub = Vec::new();
    //         rhs_pattern_mul = Vec::new();


            
    //         }  
    //     }        
    // }


    // pub fn generate_associativity_and_commutativity_rules(
    //     _optimized_rw: bool,
    //     _initial_operations: Vec<String>,
    //     _rules_info: &mut HashMap<String, Vec<String>>,
    //     _initial_rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
    //     rules: &mut Vec<Rewrite<VecLang, ConstantFold>>,
    // ) {

    //     let associativity_rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![

    //         // rewrite!("factor-1"; "(* ?a0 (+ ?b0 ?c0))" => "(+ (* ?a0 ?b0) (* ?a0 ?c0))"),

    //         rewrite!("add-assoc-3-1"; "(+ (+ ?a0 ?b0) ?c0)" => "(+ ?a0 (+ ?b0 ?c0))"),
    //         rewrite!("sub-assoc-3-1"; "(- (- ?a0 ?b0) ?c0)" => "(- ?a0 (- ?b0 ?c0))"),
    //         rewrite!("mul-assoc-3-1"; "(* (* ?a0 ?b0) ?c0)" => "(* ?a0 (* ?b0 ?c0))"),

    //         rewrite!("add-assoc-1"; "(+ (+ ?a0 ?b0) (+ ?c0 ?d0))" => "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))"),
    //         rewrite!("add-assoc-2"; "(+ (+ (+ ?a0 ?b0) ?c0) ?d0)" => "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))"),
    //         rewrite!("add-assoc-3"; "(+ ?a0 (+ (+ ?b0 ?c0) ?d0))" => "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))"),
    //         rewrite!("add-assoc-4"; "(+ (+ ?a0 (+ ?b0 ?c0)) ?d0)" => "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))"),
    //         rewrite!("add-assoc-5"; "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))" => "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))"),

    //         rewrite!("sub-assoc-1"; "(- (- ?a0 ?b0) (- ?c0 ?d0))" => "(- ?a0 (- ?b0 (- ?c0 ?d0)))"),
    //         rewrite!("sub-assoc-2"; "(- (- (- ?a0 ?b0) ?c0) ?d0)" => "(- ?a0 (- ?b0 (- ?c0 ?d0)))"),
    //         rewrite!("sub-assoc-3"; "(- ?a0 (- (- ?b0 ?c0) ?d0))" => "(- ?a0 (- ?b0 (- ?c0 ?d0)))"),
    //         rewrite!("sub-assoc-4"; "(- (- ?a0 (- ?b0 ?c0)) ?d0)" => "(- ?a0 (- ?b0 (- ?c0 ?d0)))"),
    //         rewrite!("sub-assoc-5"; "(- ?a0 (- ?b0 (- ?c0 ?d0)))" => "(- ?a0 (- ?b0 (- ?c0 ?d0)))"),

    //         rewrite!("mul-assoc-1"; "(* (* ?a0 ?b0) (* ?c0 ?d0))" => "(* ?a0 (* ?b0 (* ?c0 ?d0)))"),
    //         rewrite!("mul-assoc-2"; "(* (* (* ?a0 ?b0) ?c0) ?d0)" => "(* ?a0 (* ?b0 (* ?c0 ?d0)))"),
    //         rewrite!("mul-assoc-3"; "(* ?a0 (* (* ?b0 ?c0) ?d0))" => "(* ?a0 (* ?b0 (* ?c0 ?d0)))"),
    //         rewrite!("mul-assoc-4"; "(* (* ?a0 (* ?b0 ?c0)) ?d0)" => "(* ?a0 (* ?b0 (* ?c0 ?d0)))"),
    //         rewrite!("mul-assoc-5"; "(* ?a0 (* ?b0 (* ?c0 ?d0)))" => "(* ?a0 (* ?b0 (* ?c0 ?d0)))"),
        
    //         rewrite!("mul-split"; "(* ?a0 ?b0)" => "(VecMul (Vec ?a0 0) (Vec ?b0 0))" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //         rewrite!("mul-split"; "(+ ?a0 ?b0)" => "(VecAdd (Vec ?a0 0) (Vec ?b0 0))" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //         rewrite!("mul-split"; "(- ?a0 ?b0)" => "(VecMinus (Vec ?a0 0) (Vec ?b0 0))" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),

    //         rewrite!("balancing-1" ; "(* ?a0 (* ?b0 (+ ?c0 ?d0)))" => "(* (* ?a0 ?b0) (+ ?c0 ?d0))"),
    //         rewrite!("balancing-2" ; "(* ?a0 (* ?b0 (* ?c0 ?d0)))" => "(* (* ?a0 ?b0) (* ?c0 ?d0))"),
    //         rewrite!("balancing-3" ; "(+ ?a0 (+ ?b0 (+ ?c0 ?d0)))" => "(+ (+ ?a0 ?b0) (+ ?c0 ?d0))"),
    //         rewrite!("balancing-4" ; "(- ?a0 (- ?b0 (- ?c0 ?d0)))" => "(- (- ?a0 ?b0) (- ?c0 ?d0))"),
    //         rewrite!("balancing-5" ; "(* ?a0 (* ?b0 (* ?c0 ?d0)))" => "(* (* ?a0 ?b0) (* ?c0 ?d0))"),

    //         ];

    //         let helping_rules : Vec<Rewrite<VecLang, ConstantFold>> = vec![
    //             // rewrite!("mul-split"; "(+ ?a0 (* ?b0 ?c0))" => "(VecAdd ((VecMul (Vec ?b0 0) (Vec ?c0 0)) ?a0)" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //             // rewrite!("mul-split"; "(* ?a0 (+ ?b0 ?c0))" => "(VecMul ((VecAdd (Vec ?b0 0) (Vec ?c0 0)) ?a0)" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //             // rewrite!("scalar-to-vec-1"; "?a0" => "(Vec ?a0 0)" if are_all_symbols_or_nums(vec!["?a0".to_string()])),
    //             // rewrite!("scalar-to-vec-2"; "( + ?a0 ?b0)" => "(VecAdd (Vec ?a0 0) (Vec ?b0 0))" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //             // rewrite!("scalar-to-vec-3"; "(- ?a0 ?b0)" => "(VecMinus (Vec ?a0 0) (Vec ?b0 0))" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //             // rewrite!("scalar-to-vec-4"; "(* ?a0 ?b0)" => "(VecMul (Vec ?a0 0) (Vec ?b0 0))" if are_all_symbols_or_nums(vec!["?a0".to_string(), "?b0".to_string()])),
    //         ];

            
    //     rules.extend(associativity_rules);
    //     rules.extend(helping_rules);
        
    // }


    // fn are_all_symbols_or_nums(vars: Vec<String>) -> impl Fn(&mut EGraph<VecLang, ConstantFold>, Id, &Subst) -> bool {
    //     move |egraph: &mut EGraph<VecLang, ConstantFold>, _, subst| {
    //         vars.iter().all(|var| {
    //             let converted_var = var.as_str();
    //             let nodes = &egraph[subst[converted_var.parse().unwrap()]].nodes;
    //             nodes.iter().all(|n| matches!(n, VecLang::Symbol(_) | VecLang::Num(_)))
    //         })
    //     }
    // }
    

   