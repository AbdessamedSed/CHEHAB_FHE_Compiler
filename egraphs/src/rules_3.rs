/*
    This file contains some rules for random rotations
*/

use egg::*;
use crate::{
    veclang::{ConstantFold, VecLang},
};
use log::debug;
/*
    This file contains some rules for random rotations for multiplication.
    Rule structure:
    (Rot (Vec (* ?a0 ?b0) (* ?a1 ?b1) ... (* ?aN-1 ?bN-1)) rotation_idx)
    =>
    (VecMulRotF (Vec ?a0 ?a1 ... ?aN-1) (Vec ?b0 ?b1 ... ?bN-1) rotation_idx)
*/

pub fn are_all_symbols_or_nums(_vars: Vec<String>) -> impl Fn(&mut EGraph<VecLang, ConstantFold>, Id, &Subst) -> bool {
    move |egraph: &mut EGraph<VecLang, ConstantFold>, _, subst| {
        _vars.iter().all(|var_str| {
            let var_pattern_var: Var = var_str.parse().expect("Failed to parse var string to Var");
            let eclass_id = subst[var_pattern_var];
            let nodes = &egraph[eclass_id].nodes;
            // Check if any node in the eclass matches Symbol or Num
            nodes.iter().any(|n| matches!(n, VecLang::Symbol(_) | VecLang::Num(_)))
        })
    }
}

// --- Helper function to generate rules for a specific operation ---
pub fn generate_rot_distrib_rules_for_op(
    op_char: char,            // The binary operator: '+', '-', '*'
    vec_op_name: &str,        // Base vector op: "VecAdd", "VecMinus", "VecMul"
    vec_rot_op_name: &str,    // Rotated vector op: "VecAddRotF", "VecMinusRotF", "VecMulRotF"
    rule_op_prefix: &str,     // Prefix for rule name: "add", "sub", "mul"
    max_vector_len: usize,
) -> Vec<Rewrite<VecLang, ConstantFold>> {
    let mut rules = Vec::new();

    for num_elements in 1..=max_vector_len { // num_elements is num_products from your example
        if num_elements == 0 { continue; }

        for rotation_idx in 0..num_elements {
            // --- Construct LHS: (Rot (Vec (op ?a0 ?b0) (op ?a1 ?b1) ...) rotation_idx) ---
            let mut lhs_parts: Vec<String> = Vec::new();
            lhs_parts.push("(Rot (Vec".to_string());

            let mut pattern_vars_for_condition = Vec::new();

            for i in 0..num_elements {
                let var_a = format!("?a{}", i);
                let var_b = format!("?b{}", i);
                lhs_parts.push(format!(" ({} {} {})", op_char, var_a, var_b));
                pattern_vars_for_condition.push(var_a);
                pattern_vars_for_condition.push(var_b);
            }
            lhs_parts.push(")".to_string()); // Close (Vec ...
            lhs_parts.push(format!(" {})", rotation_idx)); // Add rotation_idx and close (Rot ...

            let lhs_str: String = lhs_parts.join("");

            // --- Construct RHS ---
            let mut rhs_parts: Vec<String> = Vec::new();
            let current_rhs_op_name: String;

            if rotation_idx == 0 {
                current_rhs_op_name = vec_op_name.to_string();
                rhs_parts.push(format!("({} (Vec", current_rhs_op_name));
                for i in 0..num_elements {
                    rhs_parts.push(format!(" ?a{}", i));
                }
                rhs_parts.push(") (Vec".to_string());
                for i in 0..num_elements {
                    rhs_parts.push(format!(" ?b{}", i));
                }
                rhs_parts.push("))".to_string());
            } else {
                current_rhs_op_name = vec_rot_op_name.to_string();
                rhs_parts.push(format!("({} (Vec", current_rhs_op_name));
                for i in 0..num_elements {
                    rhs_parts.push(format!(" ?a{}", i));
                }
                rhs_parts.push(") (Vec".to_string());
                for i in 0..num_elements {
                    rhs_parts.push(format!(" ?b{}", i));
                }
                rhs_parts.push(")".to_string());
                rhs_parts.push(format!(" {})", rotation_idx));
            }

            let rhs_str: String = rhs_parts.join("");

            let lhs_pattern: Pattern<VecLang> = match lhs_str.parse() {
                Ok(p) => p,
                Err(e) => {
                    debug!("ERROR: Failed to parse LHS pattern '{}': {:?}", lhs_str, e);
                    continue;
                }
            };
            let rhs_pattern: Pattern<VecLang> = match rhs_str.parse() {
                Ok(p) => p,
                Err(e) => {
                    debug!("ERROR: Failed to parse RHS pattern '{}': {:?}", rhs_str, e);
                    continue;
                }
            };
            
            let rule_name_suffix = if rotation_idx == 0 { vec_op_name.to_lowercase().replace("vec", "v") } else { vec_rot_op_name.to_lowercase().replace("vec", "v").replace("rotf","rot") };
            let rule_name = format!("rot-distrib-{}-{}-len{}-idx{}", rule_op_prefix, rule_name_suffix, num_elements, rotation_idx);
            
            debug!("Generated Rule: {}", rule_name);
            debug!("  LHS: {}", lhs_str);
            debug!("  RHS: {}", rhs_str);

            rules.push(rewrite!(
                rule_name;
                {lhs_pattern} => {rhs_pattern}
                if are_all_symbols_or_nums(pattern_vars_for_condition.clone())
            ));
        }
    }
    rules
}
// use egg::*;
// use crate::{
//     veclang::{ConstantFold, VecLang},
// };
// use log::debug;

// // The are_all_symbols_or_nums condition function remains the same.
// pub fn are_all_symbols_or_nums(_vars: Vec<String>) -> impl Fn(&mut EGraph<VecLang, ConstantFold>, Id, &Subst) -> bool {
//     move |egraph: &mut EGraph<VecLang, ConstantFold>, _, subst| {
//         _vars.iter().all(|var_str| {
//             let var_pattern_var: Var = var_str.parse().expect("Failed to parse var string to Var");
//             let eclass_id = subst[var_pattern_var];
//             let nodes = &egraph[eclass_id].nodes;
//             // Check if any node in the eclass matches Symbol or Num
//             nodes.iter().any(|n| matches!(n, VecLang::Symbol(_) | VecLang::Num(_)))
//         })
//     }
// }


// /// **Corrected Helper function to generate distributive rules.**
// ///
// /// This function generates rules that "distribute" an operation inside a vector
// /// out into a single vector operation.
// ///
// /// It transforms this pattern:
// ///   (Rot (Vec (op ?a0 ?b0) (op ?a1 ?b1) ...) rot_idx)
// /// Into this equivalent, more optimized pattern:
// ///   (Rot (VecOp (Vec ?a0 ?a1 ...) (Vec ?b0 ?b1 ...)) rot_idx)
// ///
// /// This is more composable and allows the e-graph to work with the
// /// `VecOp` and `Rot` nodes separately.
// pub fn generate_rot_distrib_rules_for_op(
//     op_char: char,            // The binary operator: '+', '-', '*'
//     vec_op_name: &str,        // The vector equivalent: "VecAdd", "VecMul", etc.
//     rule_op_prefix: &str,     // A prefix for the rule name for clarity: "add", "mul"
//     max_vector_len: usize,
// ) -> Vec<Rewrite<VecLang, ConstantFold>> {
//     let mut rules = Vec::new();

//     // Note: The `vec_rot_op_name` parameter is removed as it's no longer needed.

//     for num_elements in 1..=max_vector_len {
//         if num_elements == 0 { continue; }

//         for rotation_idx in 0..num_elements {
//             // --- Construct LHS: (Rot (Vec (op ?a0 ?b0) ...) rotation_idx) ---
//             // This logic is correct and remains the same.
//             let mut lhs_parts: Vec<String> = Vec::new();
//             lhs_parts.push("(Rot (Vec".to_string());

//             let mut pattern_vars_for_condition = Vec::new();

//             for i in 0..num_elements {
//                 let var_a = format!("?a{}", i);
//                 let var_b = format!("?b{}", i);
//                 lhs_parts.push(format!(" ({} {} {})", op_char, var_a, var_b));
//                 pattern_vars_for_condition.push(var_a);
//                 pattern_vars_for_condition.push(var_b);
//             }
//             lhs_parts.push(")".to_string()); // Close (Vec ...
//             lhs_parts.push(format!(" {})", rotation_idx)); // Add rotation_idx and close (Rot ...

//             let lhs_str: String = lhs_parts.join("");

//             // --- Construct RHS: (Rot (VecOp (Vec a's) (Vec b's)) rotation_idx) ---
//             // This is the corrected logic. It ALWAYS produces a composition of Rot and VecOp.
//             // There is no more `if/else` on the rotation_idx.
//             let mut rhs_parts: Vec<String> = Vec::new();
            
//             // Start the outer Rot, passing the rotation_idx to it directly.
//             rhs_parts.push(format!("(Rot ({} (Vec", vec_op_name));
//             // First vector of operands
//             for i in 0..num_elements {
//                 rhs_parts.push(format!(" ?a{}", i));
//             }
//             rhs_parts.push(") (Vec".to_string());
//             // Second vector of operands
//             for i in 0..num_elements {
//                 rhs_parts.push(format!(" ?b{}", i));
//             }
//             // Close the VecOp, then close the Rot with its rotation_idx
//             rhs_parts.push(format!(")) {})", rotation_idx));

//             let rhs_str: String = rhs_parts.join("");

//             // --- The rest of the rule creation logic is the same ---
//             let lhs_pattern: Pattern<VecLang> = match lhs_str.parse() {
//                 Ok(p) => p,
//                 Err(e) => {
//                     debug!("ERROR: Failed to parse LHS pattern '{}': {:?}", lhs_str, e);
//                     continue;
//                 }
//             };
//             let rhs_pattern: Pattern<VecLang> = match rhs_str.parse() {
//                 Ok(p) => p,
//                 Err(e) => {
//                     debug!("ERROR: Failed to parse RHS pattern '{}': {:?}", rhs_str, e);
//                     continue;
//                 }
//             };
            
//             let rule_name = format!("distrib-{}-len{}-idx{}", rule_op_prefix, num_elements, rotation_idx);
            
//             debug!("Generated Rule: {}", rule_name);
//             debug!("  LHS: {}", lhs_str);
//             debug!("  RHS: {}", rhs_str);

//             rules.push(rewrite!(
//                 rule_name;
//                 {lhs_pattern} => {rhs_pattern}
//                 if are_all_symbols_or_nums(pattern_vars_for_condition.clone())
//             ));
//         }
//     }
//     rules
// }