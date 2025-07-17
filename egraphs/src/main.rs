extern crate clap;
use clap::{App, Arg};
use egraphslib::*;
use std::time::Instant;
use crate::rules_2;
use log::debug;
use egg::*;
use crate::veclang::VecLang;
use crate::config::*;

fn main() {
    let matches = App::new("Rewriter")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("vector_width")
                .help("Sets the vector_width")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("benchmark_type")
            .help("Specify the type of the benchamark to select the rules to apply")
            .required(true)
            .index(3)
        )
        .get_matches();

    use std::{env, fs};

    let vector_width: usize = matches
        .value_of("vector_width")
        .unwrap()
        .parse()
        .expect("Number must be a valid usize");

    let benchmark_type: usize = matches
        .value_of("benchmark_type")
        .unwrap()
        .parse()
        .expect("Number must be a valid usize");

    // Get a path string to parse a program.
    let path = matches.value_of("INPUT").unwrap();
    let timeout = env::var("TIMEOUT")
        .ok()
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(300);
    let prog_str = fs::read_to_string(path).expect("Failed to read the input file.");
    eprintln!("the input expression is : {:?}", prog_str);

    
    let random_rotations = true;

    // if random_rotations {

    //     eprintln!("\nTesting: {:?}", prog_str);

    //     let infix_expr_str = match utils::prefix_to_infix_str(&prog_str) {
    //         Ok(infix) => {
    //             eprintln!("\n[MAIN] ==> Converted to Infix expr string: {:?}", infix);
    //             infix
    //         }
    //         Err(e) => {
    //             eprintln!("\n[MAIN] ==> Error converting initial prog_str to infix: {:?}", e);
    //             return; // Exit if initial conversion fails
    //         }
    //     };
    
    //     let mut rng = rand::thread_rng();
    
    //     // These are parameters for transform_randomly_pack_isomorphic_leaf_ops
    //     let num_transformations_to_show = 2; // How many times to run the first transformation
    //     let min_pack_size_for_leaves = 2;
    //     let max_pack_size_for_leaves = 10; // Max elements in a Rot node from leaf packing
    
    //     // This string is now the infix expression to be parsed by your random_rotations::Parser
    //     let mut final_infix_str_to_parse = infix_expr_str.trim().to_string();
    //     // The strip_prefix("Vec") was from your code. Ensure this is the intended logic.
    //     // If infix_expr_str could start with "Vec(...)", this removes "Vec".
    //     final_infix_str_to_parse = final_infix_str_to_parse
    //         .strip_prefix("Vec")
    //         .unwrap_or(&final_infix_str_to_parse)
    //         .to_string();
    
    //     eprintln!("[MAIN] Infix expr string after potential 'Vec' strip: {:?}", final_infix_str_to_parse);
    //     eprintln!("[MAIN] Parsing this infix string to build initial AST...");
    
    //     let mut original_parser = random_rotations::Parser::new(&final_infix_str_to_parse);
    
    //     // This will hold the AST after the *last* iteration of transform_randomly_pack_isomorphic_leaf_ops
    //     let mut last_leaf_packed_ast_opt: Option<Box<random_rotations::Expr>> = None;
    
    //     match original_parser.parse() {
    //         Ok(original_ast_root) => {
    //             eprintln!("\n[MAIN] Initial AST Structure (from parsed infix):");
    //             original_ast_root.print_tree(0, true, "".to_string());
    //             // eprintln!("\n[MAIN] Initial AST as Infix String (Display): {}", original_ast_root); // Redundant with print_tree
    
    //             for i in 0..num_transformations_to_show {
    //                 eprintln!("\n\n[MAIN] --- Leaf Packing Transformation Attempt #{} ---", i + 1);
    //                 // Clone the original_ast_root for each independent attempt of leaf packing
    //                 let ast_for_leaf_packing = original_ast_root.clone();
    
    //                 let mut current_transformed_ast = random_rotations::transform_randomly_pack_isomorphic_leaf_ops(
    //                     ast_for_leaf_packing,
    //                     min_pack_size_for_leaves,
    //                     max_pack_size_for_leaves,
    //                     &mut rng,
    //                 );

    //                 current_transformed_ast = random_rotations::expand_multiplication_by_integer(current_transformed_ast);
    
    //                 eprintln!("\n[MAIN] AST after leaf packing transformation #{}:", i + 1);
    //                 current_transformed_ast.print_tree(0, true, "".to_string());
    //                 // eprintln!("\n[MAIN] AST as Infix String (Display): {}", current_transformed_ast);
    
    //                 // Always store the latest transformed AST
    //                 last_leaf_packed_ast_opt = Some(current_transformed_ast);
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("[MAIN] Parsing Error for infix string '{}': {:?}", final_infix_str_to_parse, e);
    //         }
    //     }
    
    //     // Proceed only if the leaf packing transformation produced an AST
    //     if let Some(ast_after_leaf_packing) = last_leaf_packed_ast_opt {
    //         eprintln!("\n[MAIN] --- Post-Processing: Vectorizing Remaining Scalars ---");
    //         eprintln!("[MAIN] AST fed into scalar vectorization (from last leaf packing attempt):");
    //         ast_after_leaf_packing.print_tree(0, true, "".to_string());
    
    
    //         // Determine target_vector_width based on the ast_after_leaf_packing
    //         let rot_lengths = ast_after_leaf_packing.find_rot_node_element_lengths();
    //         let target_width_for_scalars = if !rot_lengths.is_empty() {
    //             if let Some(&max_val) = rot_lengths.iter().max() {
    //                 if max_val > 0 { max_val } else { max_pack_size_for_leaves } // Use original max if no valid rot width
    //             } else { max_pack_size_for_leaves }
    //         } else {
    //             eprintln!("[MAIN] No Rot nodes after leaf packing. Using max_pack_size_for_leaves ({}) as target_width for scalars.", max_pack_size_for_leaves);
    //             max_pack_size_for_leaves // Fallback if no Rot nodes at all
    //         };
    //         // Ensure target_width is not zero for vectorize_remaining_scalars
    //         let effective_target_width = if target_width_for_scalars == 0 {
    //             eprintln!("[MAIN] Warning: target_width_for_scalars was 0, defaulting to min_pack_size_for_leaves or 2");
    //             std::cmp::max(min_pack_size_for_leaves, 2)
    //         } else {
    //             target_width_for_scalars
    //         };
            
    //         eprintln!("[MAIN] Target vector width for scalar vectorization: {}", effective_target_width);
    
    //         let final_ast = random_rotations::vectorize_remaining_scalars(
    //             ast_after_leaf_packing, // Input is the result of the last leaf packing
    //             effective_target_width,
    //             &mut rng, // Can reuse rng
    //         );
    
    //         eprintln!("\n[MAIN] AST after Vectorizing Remaining Scalars (Final AST):");
    //         final_ast.print_tree(0, true, "".to_string());
            
    //         let prefix_string_from_final_ast = final_ast.to_prefix_string();
    //         eprintln!("\n[MAIN] Final AST as PREFIX String (to be parsed by RecExpr): {}", prefix_string_from_final_ast);
    
    //         match prefix_string_from_final_ast.parse::<RecExpr<VecLang>>() {
    //             Ok(current_expr_for_egraph) => {
    //                 debug!("[MAIN] Successfully parsed FINAL PREFIX string into RecExpr<VecLang>: {:?}", current_expr_for_egraph);
                    
    //                 let start_time = Instant::now(); // Assuming Instant is imported
                    
    //                 // `effective_target_width` is the width derived from the AST transformations
    //                 // `vector_width` (if it's a different variable from your outer scope) might be
    //                 // a configured target width for the egraph pass itself.
    //                 // You need to decide which one `rules::run` expects or benefits from.
    //                 // Let's assume `rules::run` wants the width derived from the AST.
    //                 let width_for_rules_run = effective_target_width; 
    //                 eprintln!("[MAIN] Using vector_width for rules::run: {}", width_for_rules_run);
    
    //                 // Ensure `timeout` and `vector_width` (the original one, if it's different and needed) are defined
    //                 let (cost, best) = 
    //                     rules::run(&current_expr_for_egraph, timeout, 10, width_for_rules_run, 0); // Ensure timeout is defined
                    
    //                 let duration = start_time.elapsed();
    //                 println!("{}", best.to_string()); // `println!` for final result (stdout)
                    
    //                 // This prints the width used for rules::run
    //                 println!("{} {}", width_for_rules_run, width_for_rules_run); 
                    
    //                 eprintln!("\n[MAIN] Cost: {}", cost);
    //                 eprintln!("[MAIN] Time taken in egraph: {:?} to finish", duration);
    //                 eprintln!("\n[MAIN] egraph ended");
    //             }
    //             Err(parse_to_rec_expr_error) => {
    //                 eprintln!(
    //                     "[MAIN] Failed to parse final PREFIX string ('{}') into RecExpr<VecLang>: {:?}",
    //                     prefix_string_from_final_ast,
    //                     parse_to_rec_expr_error
    //                 );
    //             }
    //         }       
    //     } else {
    //         eprintln!("[MAIN] Leaf packing transformation did not produce a final AST.");
    //     }
 

    // }

    if random_rotations {
        // ---
        // --- CONTROL FLAG: Set this to `true` for detailed step-by-step logs, or `false` for quiet execution ---
        // ---
        const VERBOSE_LOGGING: bool = true;
    
        if VERBOSE_LOGGING {
            eprintln!("\nTesting: {:?}", prog_str);
        }
    
        let infix_expr_str = match utils::prefix_to_infix_str(&prog_str) {
            Ok(infix) => {
                if VERBOSE_LOGGING {
                    eprintln!("\n[MAIN] ==> Converted to Infix expr string: {:?}", infix);
                }
                infix
            }
            Err(e) => {
                eprintln!("\n[MAIN] ==> Error converting initial prog_str to infix: {:?}", e);
                return;
            }
        };
    
        let mut rng = rand::thread_rng();
    
        // --- BEAM SEARCH CONFIGURATION (Tunable Parameters) ---
        const BEAM_WIDTH: usize = 5;
        const SEARCH_DEPTH: usize = 10;
        const BRANCHING_FACTOR: usize = 10;
    
        let min_pack_size_for_leaves = 2;
        let max_pack_size_for_leaves = 30;
    
        let mut final_infix_str_to_parse = infix_expr_str.trim().to_string();
        final_infix_str_to_parse = final_infix_str_to_parse
            .strip_prefix("Vec")
            .unwrap_or(&final_infix_str_to_parse)
            .to_string();
    
        if VERBOSE_LOGGING {
            eprintln!("[MAIN] Infix expr string after potential 'Vec' strip: {:?}", final_infix_str_to_parse);
            eprintln!("[MAIN] Parsing this infix string to build initial AST...");
        }
    
        let mut original_parser = random_rotations::Parser::new(&final_infix_str_to_parse);
    
        match original_parser.parse() {
            Ok(original_ast_root) => {
                if VERBOSE_LOGGING {
                    eprintln!("\n[MAIN] Initial AST Structure (from parsed infix):");
                    original_ast_root.print_tree(0, true, "".to_string());
                    
                    eprintln!("\n\n[MAIN] --- Starting Beam Search for Optimal Packing ---");
                    eprintln!("[MAIN] Config: Beam Width={}, Search Depth={}, Branching Factor={}", BEAM_WIDTH, SEARCH_DEPTH, BRANCHING_FACTOR);
                }
                
                let start_time = Instant::now();
                let mut beam: Vec<(Box<random_rotations::Expr>, f64)> = vec![(original_ast_root.clone(), f64::MAX)];
    
                for i in 0..SEARCH_DEPTH {
                    if VERBOSE_LOGGING {
                        eprintln!("\n[SEARCH] >>> Beam Search Iteration {}/{} <<<", i + 1, SEARCH_DEPTH);
                    }
                    let mut candidates: Vec<(Box<random_rotations::Expr>, f64)> = Vec::new();
    
                    for (ast_in_beam, _cost) in &beam {
                        for _ in 0..BRANCHING_FACTOR {
                            let candidate_ast = if i == 0 {
                                random_rotations::transform_randomly_pack_isomorphic_leaf_ops(
                                    ast_in_beam.clone(), min_pack_size_for_leaves, max_pack_size_for_leaves, &mut rng)
                            } else {
                                random_rotations::mutate_by_reshuffling_a_pack(ast_in_beam.clone(), &mut rng)
                            };
    
                            let processed_ast_for_scoring = random_rotations::expand_multiplication_by_integer(candidate_ast.clone());
                            let rot_lengths = processed_ast_for_scoring.find_rot_node_element_lengths();
                            let width = rot_lengths.iter().max().cloned().unwrap_or(max_pack_size_for_leaves);
                            let effective_width = if width == 0 { max_pack_size_for_leaves } else { width };
                            let final_ast_for_scoring = random_rotations::vectorize_remaining_scalars(
                                processed_ast_for_scoring, effective_width, &mut rng);
                            
                            let prefix_string = final_ast_for_scoring.to_prefix_string();
                            let egraph_cost = match prefix_string.parse::<RecExpr<VecLang>>() {
                                Ok(rec_expr) => {
                                    let (cost_from_run, _best_expr) = rules::run(&rec_expr, timeout, 10, effective_width, 0);
                                    cost_from_run
                                }
                                Err(_) => f64::MAX,
                            };
                            candidates.push((candidate_ast, egraph_cost));
                        }
                    }
                    
                    if VERBOSE_LOGGING {
                        eprintln!("[SEARCH] Generated and scored {} new candidates.", candidates.len());
                    }
    
                    candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                    beam = candidates.into_iter().take(BEAM_WIDTH).collect();
    
                    // This intermediate cost is always printed, as requested.
                    eprintln!("[SEARCH] Iteration {}: Top {} costs = {:?}", i + 1, BEAM_WIDTH, beam.iter().map(|(_, cost)| cost).collect::<Vec<_>>());
                }
    
                let search_duration = start_time.elapsed();
                eprintln!("\n[MAIN] Beam search complete in {:?}.", search_duration);
    
                if let Some((best_ast, final_search_cost)) = beam.into_iter().next() {
                    if VERBOSE_LOGGING {
                        eprintln!("\n[MAIN] Best packing found with a search cost of {}:", final_search_cost);
                        best_ast.print_tree(0, true, "".to_string());
                        eprintln!("\n[MAIN] --- Finalizing best AST for output ---");
                    }
                    
                    let processed_best_ast = random_rotations::expand_multiplication_by_integer(best_ast);
                    let rot_lengths = processed_best_ast.find_rot_node_element_lengths();
                    let width = rot_lengths.iter().max().cloned().unwrap_or(max_pack_size_for_leaves);
                    let final_effective_width = if width == 0 { max_pack_size_for_leaves } else { width };
                    let final_ast_for_run = random_rotations::vectorize_remaining_scalars(
                        processed_best_ast, final_effective_width, &mut rng);
                    
                    let final_prefix_string = final_ast_for_run.to_prefix_string();
                    match final_prefix_string.parse::<RecExpr<VecLang>>() {
                        Ok(rec_expr) => {
                            let (final_egraph_cost, final_best_string) = rules::run(&rec_expr, timeout, 10, final_effective_width, 0);
                            
                            // Always print the final program and its parameters to stdout.
                            println!("{}", final_best_string);
                            println!("{} {}", final_effective_width, final_effective_width);
                            
                            // Always print the final cost summary to stderr.
                            eprintln!("\n[MAIN] Final E-graph Cost: {}", final_egraph_cost);
                        },
                        Err(e) => eprintln!("[MAIN] ERROR: Could not parse the final best AST for e-graph run: {:?}", e),
                    }
                } else {
                     eprintln!("[MAIN] Beam search did not produce a final AST.");
                }
            }
            Err(e) => eprintln!("[MAIN] Parsing Error for infix string '{}': {:?}", final_infix_str_to_parse, e),
        }
    }

    else if benchmark_type == UNSTRUCTURED_WITH_ONE_OUTPUT {   // one output , unstructured
        let mut prog_str = prog_str.trim().to_string(); // Trim any leading/trailing whitespace
        debug!("remowing useless chars");
        // Remove "(Vec" at the start
        prog_str = prog_str.strip_prefix("(Vec ").unwrap_or(&prog_str).to_string(); // Remove the prefix just for rules considerations
        // Remove the last character (if it exists)
        prog_str.pop();
        prog_str.pop();

            // Print the cleaned-up expression
        debug!("The cleaned expression is: {:?}", prog_str);
        let prog = prog_str.parse().unwrap();

        // Record the start time
        let start_time = Instant::now();
       

        let (cost, best) = 
            rules::run(&prog, timeout, benchmark_type, vector_width, 0 /*rules set order is not required here*/);
        let duration = start_time.elapsed();

        // Record the end time

        // Print the results

        println!("{}", best.to_string()); /* Pretty print with width 80 */
        /*the value of the final vector width is not the follwed , this value is written in the file to facilitate the implementation
        and the real value will be calculated in the file compiler.cpp to generate the correct files after
        writing this value is important to avoid dealing with each case separaltelyn
        */
        println!("{} {}",vector_width,vector_width);
        eprintln!("\nCost: {}", cost);
        eprintln!("Time taken in egraph: {:?} to finish", duration);
        eprintln!("\negraph ended");

        
    } else {
        // if the benchamrk is structured with one output or several
        let mut current_cost = 0.0 ;
      
        let mut iteration = 0 ;
        let rulesets_appplying_order  = vec![1,2,3,4,5];
        let mut previous_cost = f64::MAX;
        let mut comp = 0;
        let mut current_expr : RecExpr<VecLang>= prog_str.parse().unwrap();
        let start_time = Instant::now();

        let mut current_vector_width = vector_width; 
        while comp != rulesets_appplying_order.len() {
            let (cost, best) = rules::run(&current_expr, timeout, benchmark_type, current_vector_width, rulesets_appplying_order[iteration%rulesets_appplying_order.len()]);
            current_expr = best ; 
            current_cost = cost ;
            current_vector_width = rules_2::get_vector_width(&current_expr);
            debug!("===> vector width in this iteration : {:?}", current_vector_width);
            if current_cost == previous_cost{
                comp+=1;
            }else{
                previous_cost=current_cost ;
                comp=0;
            }
            iteration = iteration + 1 ;
            debug!("Best cost at iteration {}: {} ", iteration + 1, current_cost);
            //eprintln!("Obtained expression ==> : {}", current_expr.to_string());
        }
        let best_cost = current_cost ;
        let best_expr = current_expr.clone(); 
        // Print the results
        let duration = start_time.elapsed();
        println!("{}", best_expr.to_string()); /* Pretty print with width 80 */
        debug!("best expression : {:?}", best_expr.to_string());
        println!("{} {}",current_vector_width,current_vector_width);
        eprintln!("\nCost: {}", best_cost);
        eprintln!("Time taken in egraph: {:?} to finish", duration);
        eprintln!("\negraph ended");
        
    }
}
