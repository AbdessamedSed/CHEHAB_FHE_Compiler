use std::usize;
use egg::*;
use crate::{
    extractor::GreedyExtractor, 
    extractor_sa::SimulatedAnnealingExtractor,
    extractor_exhaustive::ExhaustiveExtractor,     veclang::{ConstantFold, Egraph, VecLang},
    runner::Runner,
    cost::VecCostFn,
    // lp_extract:: {LpExtractor, LpCostFunction}
};
use crate::rules_1;
use crate::rules_2;
use std::collections::HashMap;
use std::collections::HashSet;
use log::debug;
use crate::config::*;
// Check if all the variables, in this case memories, are equivalent

/// Run the rewrite rules over the input program and return the best (cost, program)
use std::time::Instant;

pub fn run(
    prog: &RecExpr<VecLang>,
    timeout: u64,
    benchmark_type: usize,
    vector_width: usize,
    selected_ruleset_order : usize, // this parameter is needed for the second version of the rules , to vectorize the structured code
) -> (f64, RecExpr<VecLang>) {
    
    let rules_info : HashMap<String, Vec<String>> = HashMap::new(); // this is for optimization techinque, it is not used at this level
    let initial_rules : Vec<Rewrite<VecLang, ConstantFold>> = Vec::new(); // idem
    let mut rules : Vec<Rewrite<VecLang, ConstantFold>> = Vec::new();
    if benchmark_type == UNSTRUCTURED_WITH_ONE_OUTPUT {   // One output, not structured
        eprintln!("unstructured code with one output");
        rules_1::generate_rules_unstructured_code(
            &mut rules
        );
    
        // rules_1::generate_associativity_and_commutativity_rules(
        //     &mut rules
        // );

    } else if benchmark_type == STRUCTURED_WITH_ONE_OUTPUT || benchmark_type == STRUCTURED_WITH_MULTIPLE_OUTPUTS {
        eprintln!("structured code with one output or multiple outpits");
        debug!("vector width is {:?}", vector_width);
        let expression_depth : usize = rules_2::ast_depth(&prog);
        debug!("depth of the expression is : {:?}", expression_depth);
        match selected_ruleset_order {
            1 => {
                rules.extend(rules_2::addition_rules(vector_width, expression_depth));

            }
            2 => {
                rules.extend(rules_2::minus_rules(vector_width,expression_depth));        
            }
            3 => {
                rules.extend(rules_2::multiplication_rules(vector_width,expression_depth));         
            },
            4 => {
                rules.extend(rules_2::neg_rules(vector_width,expression_depth));
            },
            5 => {
                rules.extend(rules_2::vector_assoc_min_rules());
                rules.extend(rules_2::vector_assoc_mul_rules());
                rules.extend(rules_2::vector_assoc_add_rules());
                rules.extend(rules_2::vector_assoc_add_mul_rules());
                rules.extend(rules_2::vector_assoc_add_min_rules());
                rules.extend(rules_2::vector_assoc_min_mul_rules());
                
            },

            _ => debug!("Ruleset correspoding to this order doesnt exist"),
        }
    }

    // Start timing the e-graph building process
    let start_time = Instant::now();

    // Initialize the e-graph with constant folding enabled and add a zero literal
    let mut init_eg = Egraph::new(ConstantFold);
    init_eg.add(VecLang::Num(0));

    type MyRunner = Runner<VecLang, ConstantFold>;


    let runner = MyRunner::new(Default::default())
        .with_egraph(init_eg)
        .with_expr(&prog)
        .with_node_limit(100_000)
        .with_time_limit(std::time::Duration::from_secs(timeout))
        .with_iter_limit(10_000)
        // .run(&rules);
        .run(&rules, &initial_rules, rules_info, /*optimized rules*/false);    
            // the last flage to enable an optimization technique developed within egg
            // it is set to false in this case

        let report = runner.report();
        eprintln!("report : {:?}", report);
        /* for the rules , if the rule is expensive we add the prefix exp to its name */


    // Stop timing after the e-graph is built
    let build_time = start_time.elapsed();
    debug!("E-graph built in {:?}", build_time);

    // Print the reason for stopping to STDERR
    debug!(
        "Stopped after {} iterations, reason: {:?}",
        runner.iterations.len(),
        runner.stop_reason
    );

   

    // Extract the e-graph and the root node
    let (eg, root) = (runner.egraph, runner.roots[0]);
    debug!("final number of enodes : {:?}", eg.total_size());
    // print_egraph(eg.clone());

    let extraction_technic = 0;
    let mut best_cost: f64 = 0.0;
    let mut best_expr : RecExpr<VecLang> = RecExpr::default();
    debug!("begining of extraction 0 .... ");

    /* we have 3 ways fot the extraction:
        1) greedy_extraction: takes decisions locally
        2) exhaustive_extraction: exploring all possibilities
        3) sa_extraction: based on simulating annealing metaheuristic
    */

    /*********************************** integer linear programming extraction **********************/
    // let cost_fn = AstSize {}; 
    // let start_extract_time = Instant::now();
    // let mut extractor = LpExtractor::new(&eg, cost_fn);
    // extractor.timeout(300.0);
    // (best_expr) = extractor.solve(root);
    // let extract_time = start_extract_time.elapsed();
   
    /************************************ greedy extraction ******************************************/
    if extraction_technic == 0 {
        let start_extract_time = Instant::now();
        let mut extractor = GreedyExtractor::new(&eg, VecCostFn { egraph: &eg }, root, false);
        (best_cost, best_expr) = extractor.find_best(root);
        let _extract_time = start_extract_time.elapsed();
    }else if extraction_technic == 1 {
        /********************************** Exhaustive extraction *************************************/
        let start_extract_time = Instant::now();
        let mut extractor = ExhaustiveExtractor::new(&eg);
        extractor.find_best(
            vec![root],
            HashMap::new(),
            root,
            0,
            0.0,
            vec![],
            &mut best_cost,
            &mut best_expr,
            &mut HashMap::new(), 
        );
        let _extract_time = start_extract_time.elapsed();
        /******************************************************************************/
    }else if extraction_technic == 2 {
        /************************************ SA extraction *************************************************/
        let start_extract_time = Instant::now();
        let mut extractor = SimulatedAnnealingExtractor::new(&eg);
        let mut _n_cost:usize = 0;
        // // Parameters for simulated annealing
        let max_iteration = 200000;
        let initial_temp = 200000.0;
        let cooling_rate = 0.995;
        (best_cost, best_expr) = extractor.find_best(
            &eg,
            root,
            max_iteration,
            initial_temp,
            cooling_rate,
        );
        let _extract_time = start_extract_time.elapsed();
        //Stop timing after the extraction is complete
        eprintln!("display final results");
        /********************************************************************************/
    }
    // Stop timing after the extraction is complete
    debug!("display final results");
    debug!("Final cost is {}", best_cost);
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
