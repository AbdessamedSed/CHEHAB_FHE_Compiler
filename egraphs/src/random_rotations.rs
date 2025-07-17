use std::collections::{HashMap, HashSet};
use std::fmt;
use rand::seq::SliceRandom;
use rand::Rng;
use log::debug;

// --- AST Node Definitions ---
#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    BinOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Rot {
        rotation_index: usize,
        elements: Vec<Box<Expr>>, // These are the expressions in the "Vec"
    },
}

impl Expr {
    // Helper for cloning Box<Expr> for elements in RotNode
    pub fn clone_box_elements(elements: &[Box<Expr>]) -> Vec<Box<Expr>> {
        elements.iter().map(|e| e.clone()).collect()
    }

    pub fn print_tree(&self, indent: usize, is_last: bool, prefix: String) {
        let base_prefix = prefix.clone();
        let current_prefix = if indent > 0 {
            if is_last { "└── " } else { "├── " }
        } else {
            ""
        };
        debug!("{}{}", base_prefix, current_prefix);

        match self {
            Expr::Var(name) => debug!("Var({})", name),
            Expr::BinOp { op, left, right } => {
                debug!("Op({})", op);
                let child_prefix = prefix.clone() + (if is_last { "    " } else { "│   " });
                left.print_tree(indent + 1, false, child_prefix.clone());
                right.print_tree(indent + 1, true, child_prefix);
            }
            Expr::Rot { rotation_index, elements } => {
                // Build the (Vec element1 element2 ...) string part
                let mut elements_str_for_rot_line = String::from("(Vec");
                for elem_box in elements {
                    elements_str_for_rot_line.push(' ');
                    elements_str_for_rot_line.push_str(&elem_box.to_string()); // Uses Display for each element
                }
                elements_str_for_rot_line.push(')');

                // Print the Rot node line in the new desired format
                debug!("(Rot {} {})", elements_str_for_rot_line, rotation_index);
                
                // The children (elements list) are still printed underneath for tree view clarity
                let child_prefix = prefix + (if is_last { "    " } else { "│   " });
                // Visual cue that these are elements of the Rot node just printed
                if !elements.is_empty() {
                    debug!("{}  └── Elements List (for above Rot):", child_prefix);
                    let element_list_base_prefix = child_prefix.clone() + "      "; // Indent further under "Elements List"
                    
                    for (i, elem) in elements.iter().enumerate() {
                        let is_last_element = i == elements.len() - 1;
                        let element_prefix_arm = if is_last_element { "└── " } else { "├── " };
                        
                        let selected_info = if i == *rotation_index {
                            " (conceptually selected by this Rot)"
                        } else {
                            ""
                        };
                        // Print the index and selection status on one line
                        debug!("{}{}[{}]:{}", element_list_base_prefix, element_prefix_arm, i, selected_info);
                        // println!(); // Newline before printing the element's tree
                        
                        // Prepare prefix for the element's own tree structure
                        let sub_element_prefix = element_list_base_prefix.clone() + (if is_last_element { "    " } else { "│   " });
                        elem.print_tree(indent + 3, true, sub_element_prefix); // Pass true for is_last to avoid extending connector
                    }
                }
            }
        }
    }

    pub fn to_prefix_string(&self) -> String {
        match self {
            Expr::Var(name) => name.clone(),
            Expr::BinOp { op, left, right } => {
                format!("({} {} {})", op, left.to_prefix_string(), right.to_prefix_string())
            }
            Expr::Rot { rotation_index, elements } => {
                let mut elements_prefix_str = String::new();
                for (i, elem_box) in elements.iter().enumerate() {
                    if i > 0 {
                        elements_prefix_str.push(' ');
                    }
                    elements_prefix_str.push_str(&elem_box.to_prefix_string());
                }
                format!("(Rot (Vec {}) {})", elements_prefix_str, rotation_index)
            }
        }
    }

    pub fn find_rot_node_element_lengths(&self) -> Vec<usize> {
        let mut lengths = Vec::new();
        self.collect_rot_element_lengths_recursive(&mut lengths);
        lengths
    }

    fn collect_rot_element_lengths_recursive(&self, lengths: &mut Vec<usize>) {
        match self {
            Expr::Var(_) => {}
            Expr::BinOp { left, right, .. } => {
                left.collect_rot_element_lengths_recursive(lengths);
                right.collect_rot_element_lengths_recursive(lengths);
            }
            Expr::Rot { elements, .. } => {
                lengths.push(elements.len());
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{}", name),
            Expr::BinOp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expr::Rot { rotation_index, elements } => {
                write!(f, "(Rot (Vec")?;
                for elem_box in elements {
                    write!(f, " {}", elem_box)?;
                }
                write!(f, ") {})", rotation_index)
            }
        }
    }
}


// --- Parser ---
#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseError(String);

pub struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input_str: &'a str) -> Self {
        Parser {
            input: input_str.as_bytes(),
            pos: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.skip_whitespace();
        if self.pos < self.input.len() {
            Some(self.input[self.pos] as char)
        } else {
            None
        }
    }

    fn consume(&mut self) -> Result<char, ParseError> {
        self.skip_whitespace();
        if self.pos < self.input.len() {
            let ch = self.input[self.pos] as char;
            self.pos += 1;
            Ok(ch)
        } else {
            Err(ParseError("Unexpected end of input".to_string()))
        }
    }

    fn expect(&mut self, expected: char) -> Result<(), ParseError> {
        let ch = self.consume()?;
        if ch == expected {
            Ok(())
        } else {
            Err(ParseError(format!(
                "Expected '{}' but found '{}' at pos {}",
                expected, ch, self.pos
            )))
        }
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, ParseError> {
        self.skip_whitespace();
        match self.peek() {
            Some(c) if c.is_alphabetic() || c.is_digit(10) => { // Allow numbers as factors
                let mut name = String::new();
                while let Some(ch_val) = self.peek() {
                    if ch_val.is_alphanumeric() || ch_val == '_' {
                        name.push(self.consume()?);
                    } else {
                        break;
                    }
                }
                Ok(Box::new(Expr::Var(name)))
            }
            Some('(') => {
                self.expect('(')?;
                let expr = self.parse_expression()?;
                self.expect(')')?;
                Ok(expr)
            }
            Some(other) => Err(ParseError(format!("Unexpected token '{}' at pos {}", other, self.pos))),
            None => Err(ParseError("Unexpected end of input in factor".to_string())),
        }
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut node = self.parse_factor()?;
        self.skip_whitespace();
        while let Some(op_char @ ('*' | '/')) = self.peek() {
            self.consume()?; 
            let right = self.parse_factor()?;
            node = Box::new(Expr::BinOp {
                op: op_char,
                left: node,
                right,
            });
            self.skip_whitespace();
        }
        Ok(node)
    }

    fn parse_expression(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut node = self.parse_term()?;
        self.skip_whitespace();
        while let Some(op_char @ ('+' | '-')) = self.peek() {
            self.consume()?;
            let right = self.parse_term()?;
            node = Box::new(Expr::BinOp {
                op: op_char,
                left: node,
                right,
            });
            self.skip_whitespace();
        }
        Ok(node)
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParseError> {
        self.pos = 0;
        let result = self.parse_expression()?;
        self.skip_whitespace();
        if self.pos < self.input.len() && self.input[self.pos] != b')' {
             Err(ParseError(format!(
                "Unexpected characters at end of expression: '{}'",
                String::from_utf8_lossy(&self.input[self.pos..])
            )))
        } else {
            Ok(result)
        }
    }
}


// --- Transformation Logic ---
type SlotPtr = *mut Box<Expr>;
type NodeRawPtr = *const Expr;

pub fn collect_packable_leaf_operations_recursive(
    current_node_slot: SlotPtr,
    candidates: &mut HashMap<char, Vec<SlotPtr>>,
    visited_raw_nodes: &mut HashSet<NodeRawPtr>,
) {
    let current_node_box: &mut Box<Expr> = unsafe { &mut *current_node_slot };
    let current_node_raw: NodeRawPtr = &**current_node_box as NodeRawPtr;

    if visited_raw_nodes.contains(&current_node_raw) {
        return;
    }
    visited_raw_nodes.insert(current_node_raw);

    if let Expr::BinOp { op, ref mut left, ref mut right } = **current_node_box {
        let mut left_is_var = false;
        if let Expr::Var(_) = **left {
            left_is_var = true;
        }
        let mut right_is_var = false;
        if let Expr::Var(_) = **right {
            right_is_var = true;
        }

        if left_is_var && right_is_var {
            candidates.entry(op).or_default().push(current_node_slot);
        }

        collect_packable_leaf_operations_recursive(left as *mut Box<Expr>, candidates, visited_raw_nodes);
        collect_packable_leaf_operations_recursive(right as *mut Box<Expr>, candidates, visited_raw_nodes);
    }
}

pub fn transform_randomly_pack_isomorphic_leaf_ops(
    mut root: Box<Expr>,
    min_pack_size: usize,
    max_pack_size: usize,
    rng: &mut impl Rng,
) -> Box<Expr> {
    if min_pack_size < 2 { panic!("min_pack_size must be at least 2"); }

    let mut slots_already_holding_rot: HashSet<SlotPtr> = HashSet::new();
    let mut original_nodes_consumed_as_rot_elements: HashSet<NodeRawPtr> = HashSet::new();
    
    let root_slot_ptr: SlotPtr = &mut root as *mut Box<Expr>;

    let mut initial_op_types_map: HashMap<char, Vec<SlotPtr>> = HashMap::new();
    let mut temp_visited_for_op_scan: HashSet<NodeRawPtr> = HashSet::new();
    collect_packable_leaf_operations_recursive(root_slot_ptr, &mut initial_op_types_map, &mut temp_visited_for_op_scan);
    
    let op_types_to_process: Vec<char> = initial_op_types_map.keys().cloned().collect();

    for &op_char in &op_types_to_process {
        loop { 
            let mut made_a_pack_this_op_type = false;

            let mut current_candidates_map: HashMap<char, Vec<SlotPtr>> = HashMap::new();
            let mut visited_for_this_collection_pass: HashSet<NodeRawPtr> = HashSet::new();
            collect_packable_leaf_operations_recursive(root_slot_ptr, &mut current_candidates_map, &mut visited_for_this_collection_pass);
            
            let mut available_slots_for_pack: Vec<SlotPtr> = Vec::new();
            if let Some(slots_for_op) = current_candidates_map.get(&op_char) {
                for &slot_ptr in slots_for_op {
                    let raw_node_at_slot: NodeRawPtr = unsafe { &**slot_ptr as NodeRawPtr };
                    if !slots_already_holding_rot.contains(&slot_ptr) &&
                       !original_nodes_consumed_as_rot_elements.contains(&raw_node_at_slot) {
                        available_slots_for_pack.push(slot_ptr);
                    }
                }
            }

            if available_slots_for_pack.len() < min_pack_size {
                break; 
            }

            available_slots_for_pack.shuffle(rng);

            let current_max_packable = std::cmp::min(max_pack_size, available_slots_for_pack.len());
            if current_max_packable < min_pack_size { 
                break;
            }
            let pack_n = rng.gen_range(min_pack_size..=current_max_packable);
            
            if pack_n == 0 || available_slots_for_pack.len() < pack_n { 
                continue;
            }

            let slots_for_current_pack: Vec<SlotPtr> = available_slots_for_pack.iter().take(pack_n).cloned().collect();

            debug!("Attempting to pack {} leaf-operations of type '{}'", pack_n, op_char);

            let mut snapshot_of_original_nodes_for_rot_elements: Vec<Box<Expr>> = Vec::new();
            for &slot_ptr_for_snapshot in &slots_for_current_pack {
                let original_node_ref: &Expr = unsafe { &**slot_ptr_for_snapshot };
                let original_node_raw_ptr: NodeRawPtr = original_node_ref as NodeRawPtr;
                
                snapshot_of_original_nodes_for_rot_elements.push(Box::new(original_node_ref.clone()));                
                original_nodes_consumed_as_rot_elements.insert(original_node_raw_ptr);
                debug!("  Adding to pack (snapshot): {}", original_node_ref);
            }

            for (i, &target_slot_to_replace) in slots_for_current_pack.iter().enumerate() {
                let mut rot_elements_for_this_rot_instance: Vec<Box<Expr>> = Vec::new();
                for original_node_snapshot_box in &snapshot_of_original_nodes_for_rot_elements {
                    rot_elements_for_this_rot_instance.push(original_node_snapshot_box.clone());
                }

                let new_rot_node_expr = Expr::Rot {
                    rotation_index: i,
                    elements: rot_elements_for_this_rot_instance,
                };
                
                debug!("  Replacing node {} with Rot{}(...)", unsafe { &**target_slot_to_replace }, i);
                unsafe {
                    *target_slot_to_replace = Box::new(new_rot_node_expr);
                }
                slots_already_holding_rot.insert(target_slot_to_replace);
            }
            made_a_pack_this_op_type = true;

            if !made_a_pack_this_op_type { 
                break;
            }
        }
    }
    root 
}

// **************************************************************************
// NEW FUNCTION TO SOLVE THE PARSING PROBLEM
// **************************************************************************

/// Recursively traverses the AST to find multiplications by an integer literal
/// and expands them into a sum of terms.
///
/// For example, `(expr) * 3` becomes `((expr) + (expr)) + (expr)`.
/// This is necessary because the target infix parser cannot handle `expr * literal`.
pub fn expand_multiplication_by_integer(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        // Base case: Variables are returned as is.
        Expr::Var(_) => expr,

        // Recursive case: For Rot nodes, apply the expansion to each element.
        Expr::Rot { rotation_index, elements } => {
            let new_elements = elements.into_iter()
                .map(expand_multiplication_by_integer)
                .collect();
            Box::new(Expr::Rot { rotation_index, elements: new_elements })
        }

        // Main logic: Check BinOp nodes.
        Expr::BinOp { op, left, right } => {
            // First, recurse on children to expand from the bottom up.
            let new_left = expand_multiplication_by_integer(left);
            let new_right = expand_multiplication_by_integer(right);

            // Check if the current operation is a multiplication.
            if op == '*' {
                // Determine if one side is a number and the other is the expression to expand.
                let mut num_opt: Option<u32> = None;
                let mut expr_to_expand_opt: Option<Box<Expr>> = None;

                // Case 1: (expr * number)
                if let Expr::Var(s) = &*new_right {
                    if let Ok(n) = s.parse::<u32>() {
                        num_opt = Some(n);
                        expr_to_expand_opt = Some(new_left.clone());
                    }
                }
                // Case 2: (number * expr)
                else if let Expr::Var(s) = &*new_left {
                     if let Ok(n) = s.parse::<u32>() {
                        num_opt = Some(n);
                        expr_to_expand_opt = Some(new_right.clone());
                    }
                }

                // If we found a match (expr * N or N * expr)
                if let (Some(n), Some(the_expr)) = (num_opt, expr_to_expand_opt) {
                    debug!("Expanding multiplication: ({}) * {}", the_expr, n);
                    
                    // Handle the expansion based on the value of N.
                    return match n {
                        0 => Box::new(Expr::Var("0".to_string())), // expr * 0 = 0
                        1 => the_expr, // expr * 1 = expr
                        _ => { // n > 1, build the sum tree
                            let mut sum_tree = the_expr.clone(); // Start with the first term
                            // Add (n-1) more terms
                            for _ in 1..n {
                                sum_tree = Box::new(Expr::BinOp {
                                    op: '+',
                                    left: sum_tree,
                                    right: the_expr.clone(),
                                });
                            }
                            sum_tree
                        }
                    };
                }
            }

            // If it's not a multiplication by a number, just rebuild the node
            // with its (potentially transformed) children.
            Box::new(Expr::BinOp { op, left: new_left, right: new_right })
        }
    }
}


// use std::collections::{HashMap, HashSet};
// use std::fmt;
// use rand::seq::SliceRandom;
// use rand::Rng;
// use log::debug;

// // --- AST Node Definitions ---
// #[derive(Debug, Clone)]
// pub enum Expr {
//     Var(String),
//     BinOp {
//         op: char,
//         left: Box<Expr>,
//         right: Box<Expr>,
//     },
//     Rot {
//         rotation_index: usize,
//         elements: Vec<Box<Expr>>, // These are the expressions in the "Vec"
//     },
// }

// impl Expr {
//     // Helper for cloning Box<Expr> for elements in RotNode
//     pub fn clone_box_elements(elements: &[Box<Expr>]) -> Vec<Box<Expr>> {
//         elements.iter().map(|e| e.clone()).collect()
//     }

//     pub fn print_tree(&self, indent: usize, is_last: bool, prefix: String) {
//         let base_prefix = prefix.clone();
//         let current_prefix = if indent > 0 {
//             if is_last { "└── " } else { "├── " }
//         } else {
//             ""
//         };
//         debug!("{}{}", base_prefix, current_prefix);

//         match self {
//             Expr::Var(name) => debug!("Var({})", name),
//             Expr::BinOp { op, left, right } => {
//                 debug!("Op({})", op);
//                 let child_prefix = prefix.clone() + (if is_last { "    " } else { "│   " });
//                 left.print_tree(indent + 1, false, child_prefix.clone());
//                 right.print_tree(indent + 1, true, child_prefix);
//             }
//             Expr::Rot { rotation_index, elements } => {
//                 // Build the (Vec element1 element2 ...) string part
//                 let mut elements_str_for_rot_line = String::from("(Vec");
//                 for elem_box in elements {
//                     elements_str_for_rot_line.push(' ');
//                     elements_str_for_rot_line.push_str(&elem_box.to_string()); // Uses Display for each element
//                 }
//                 elements_str_for_rot_line.push(')');

//                 // Print the Rot node line in the new desired format
//                 debug!("(Rot {} {})", elements_str_for_rot_line, rotation_index);
                
//                 // The children (elements list) are still printed underneath for tree view clarity
//                 let child_prefix = prefix + (if is_last { "    " } else { "│   " });
//                 // Visual cue that these are elements of the Rot node just printed
//                 if !elements.is_empty() {
//                     debug!("{}  └── Elements List (for above Rot):", child_prefix);
//                     let element_list_base_prefix = child_prefix.clone() + "      "; // Indent further under "Elements List"
                    
//                     for (i, elem) in elements.iter().enumerate() {
//                         let is_last_element = i == elements.len() - 1;
//                         let element_prefix_arm = if is_last_element { "└── " } else { "├── " };
                        
//                         let selected_info = if i == *rotation_index {
//                             " (conceptually selected by this Rot)"
//                         } else {
//                             ""
//                         };
//                         // Print the index and selection status on one line
//                         debug!("{}{}[{}]:{}", element_list_base_prefix, element_prefix_arm, i, selected_info);
//                         // println!(); // Newline before printing the element's tree
                        
//                         // Prepare prefix for the element's own tree structure
//                         let sub_element_prefix = element_list_base_prefix.clone() + (if is_last_element { "    " } else { "│   " });
//                         elem.print_tree(indent + 3, true, sub_element_prefix); // Pass true for is_last to avoid extending connector
//                     }
//                 }
//             }
//         }
//     }

//     pub fn to_prefix_string(&self) -> String {
//         match self {
//             Expr::Var(name) => name.clone(),
//             Expr::BinOp { op, left, right } => {
//                 format!("({} {} {})", op, left.to_prefix_string(), right.to_prefix_string())
//             }
//             Expr::Rot { rotation_index, elements } => {
//                 let mut elements_prefix_str = String::new();
//                 for (i, elem_box) in elements.iter().enumerate() {
//                     if i > 0 {
//                         elements_prefix_str.push(' ');
//                     }
//                     elements_prefix_str.push_str(&elem_box.to_prefix_string());
//                 }
//                 format!("(Rot (Vec {}) {})", elements_prefix_str, rotation_index)
//             }
//         }
//     }

//     pub fn find_rot_node_element_lengths(&self) -> Vec<usize> {
//         let mut lengths = Vec::new();
//         self.collect_rot_element_lengths_recursive(&mut lengths);
//         lengths
//     }

//     fn collect_rot_element_lengths_recursive(&self, lengths: &mut Vec<usize>) {
//         match self {
//             Expr::Var(_) => {}
//             Expr::BinOp { left, right, .. } => {
//                 left.collect_rot_element_lengths_recursive(lengths);
//                 right.collect_rot_element_lengths_recursive(lengths);
//             }
//             Expr::Rot { elements, .. } => {
//                 lengths.push(elements.len());
//                 // Typically, elements of a Rot node are base operations,
//                 // but if Rot nodes could be nested within elements (unlikely for your current setup),
//                 // you might recurse here too. For now, just collect current Rot's element length.
//                 // for elem in elements {
//                 //     elem.collect_rot_element_lengths_recursive(lengths);
//                 // }
//             }
//         }
//     }
// }

// impl fmt::Display for Expr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Expr::Var(name) => write!(f, "{}", name),
//             Expr::BinOp { op, left, right } => {
//                 write!(f, "({} {} {})", left, op, right)
//             }
//             Expr::Rot { rotation_index, elements } => {
//                 // New format: (Rot (Vec element1 element2 ...) rotation_index)
//                 write!(f, "(Rot (Vec")?;
//                 for elem_box in elements {
//                     write!(f, " {}", elem_box)?; // elem_box.to_string() uses its own Display
//                 }
//                 write!(f, ") {})", rotation_index)
//             }
//         }
//     }
// }


// // --- Parser ---
// #[derive(Debug)]
// #[allow(dead_code)] // To silence warning if ParseError fields are only used by Debug
// pub struct ParseError(String);

// pub struct Parser<'a> {
//     input: &'a [u8],
//     pos: usize,
// }

// impl<'a> Parser<'a> {
//     pub fn new(input_str: &'a str) -> Self {
//         Parser {
//             input: input_str.as_bytes(),
//             pos: 0,
//         }
//     }

//     fn skip_whitespace(&mut self) {
//         while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
//             self.pos += 1;
//         }
//     }

//     fn peek(&mut self) -> Option<char> {
//         self.skip_whitespace();
//         if self.pos < self.input.len() {
//             Some(self.input[self.pos] as char)
//         } else {
//             None
//         }
//     }

//     fn consume(&mut self) -> Result<char, ParseError> {
//         self.skip_whitespace();
//         if self.pos < self.input.len() {
//             let ch = self.input[self.pos] as char;
//             self.pos += 1;
//             Ok(ch)
//         } else {
//             Err(ParseError("Unexpected end of input".to_string()))
//         }
//     }

//     fn expect(&mut self, expected: char) -> Result<(), ParseError> {
//         let ch = self.consume()?;
//         if ch == expected {
//             Ok(())
//         } else {
//             Err(ParseError(format!(
//                 "Expected '{}' but found '{}' at pos {}",
//                 expected, ch, self.pos
//             )))
//         }
//     }

//     fn parse_factor(&mut self) -> Result<Box<Expr>, ParseError> {
//         self.skip_whitespace();
//         match self.peek() {
//             Some(c) if c.is_alphabetic() => {
//                 let mut name = String::new();
//                 name.push(self.consume()?); 
//                 while let Some(ch_val) = self.peek() {
//                     if ch_val.is_alphanumeric() || ch_val == '_' {
//                         name.push(self.consume()?);
//                     } else {
//                         break;
//                     }
//                 }
//                 Ok(Box::new(Expr::Var(name)))
//             }
//             Some('(') => {
//                 self.expect('(')?;
//                 let expr = self.parse_expression()?;
//                 self.expect(')')?;
//                 Ok(expr)
//             }
//             Some(other) => Err(ParseError(format!("Unexpected token '{}' at pos {}", other, self.pos))),
//             None => Err(ParseError("Unexpected end of input in factor".to_string())),
//         }
//     }

//     fn parse_term(&mut self) -> Result<Box<Expr>, ParseError> {
//         let mut node = self.parse_factor()?;
//         self.skip_whitespace();
//         while let Some(op_char @ ('*' | '/')) = self.peek() {
//             self.consume()?; 
//             let right = self.parse_factor()?;
//             node = Box::new(Expr::BinOp {
//                 op: op_char,
//                 left: node,
//                 right,
//             });
//             self.skip_whitespace();
//         }
//         Ok(node)
//     }

//     fn parse_expression(&mut self) -> Result<Box<Expr>, ParseError> {
//         let mut node = self.parse_term()?;
//         self.skip_whitespace();
//         while let Some(op_char @ ('+' | '-')) = self.peek() {
//             self.consume()?;
//             let right = self.parse_term()?;
//             node = Box::new(Expr::BinOp {
//                 op: op_char,
//                 left: node,
//                 right,
//             });
//             self.skip_whitespace();
//         }
//         Ok(node)
//     }

//     pub fn parse(&mut self) -> Result<Box<Expr>, ParseError> {
//         self.pos = 0;
//         let result = self.parse_expression()?;
//         self.skip_whitespace();
//         if self.pos < self.input.len() && self.input[self.pos] != b')' {
//              Err(ParseError(format!(
//                 "Unexpected characters at end of expression: '{}'",
//                 String::from_utf8_lossy(&self.input[self.pos..])
//             )))
//         } else {
//             Ok(result)
//         }
//     }
// }


// // --- Transformation Logic ---
// type SlotPtr = *mut Box<Expr>;
// type NodeRawPtr = *const Expr;

// pub fn collect_packable_leaf_operations_recursive(
//     current_node_slot: SlotPtr,
//     candidates: &mut HashMap<char, Vec<SlotPtr>>,
//     visited_raw_nodes: &mut HashSet<NodeRawPtr>,
// ) {
//     let current_node_box: &mut Box<Expr> = unsafe { &mut *current_node_slot };
//     let current_node_raw: NodeRawPtr = &**current_node_box as NodeRawPtr;

//     if visited_raw_nodes.contains(&current_node_raw) {
//         return;
//     }
//     visited_raw_nodes.insert(current_node_raw);

//     if let Expr::BinOp { op, ref mut left, ref mut right } = **current_node_box {
//         let mut left_is_var = false;
//         if let Expr::Var(_) = **left {
//             left_is_var = true;
//         }
//         let mut right_is_var = false;
//         if let Expr::Var(_) = **right {
//             right_is_var = true;
//         }

//         if left_is_var && right_is_var {
//             candidates.entry(op).or_default().push(current_node_slot);
//         }

//         collect_packable_leaf_operations_recursive(left as *mut Box<Expr>, candidates, visited_raw_nodes);
//         collect_packable_leaf_operations_recursive(right as *mut Box<Expr>, candidates, visited_raw_nodes);
//     }
// }

// pub fn transform_randomly_pack_isomorphic_leaf_ops(
//     mut root: Box<Expr>,
//     min_pack_size: usize,
//     max_pack_size: usize,
//     rng: &mut impl Rng,
// ) -> Box<Expr> {
//     if min_pack_size < 2 { panic!("min_pack_size must be at least 2"); }

//     let mut slots_already_holding_rot: HashSet<SlotPtr> = HashSet::new();
//     let mut original_nodes_consumed_as_rot_elements: HashSet<NodeRawPtr> = HashSet::new();
    
//     let root_slot_ptr: SlotPtr = &mut root as *mut Box<Expr>;

//     let mut initial_op_types_map: HashMap<char, Vec<SlotPtr>> = HashMap::new();
//     let mut temp_visited_for_op_scan: HashSet<NodeRawPtr> = HashSet::new();
//     collect_packable_leaf_operations_recursive(root_slot_ptr, &mut initial_op_types_map, &mut temp_visited_for_op_scan);
    
//     let op_types_to_process: Vec<char> = initial_op_types_map.keys().cloned().collect();

//     for op_char_ref in op_types_to_process { // Iterate by reference to avoid borrow checker issues with rng
//         let op_char = op_char_ref; // Dereference to get the char
//         let mut made_a_pack_this_op_type;
//         loop { 
//             made_a_pack_this_op_type = false;

//             let mut current_candidates_map: HashMap<char, Vec<SlotPtr>> = HashMap::new();
//             let mut visited_for_this_collection_pass: HashSet<NodeRawPtr> = HashSet::new();
//             collect_packable_leaf_operations_recursive(root_slot_ptr, &mut current_candidates_map, &mut visited_for_this_collection_pass);
            
//             let mut available_slots_for_pack: Vec<SlotPtr> = Vec::new();
//             if let Some(slots_for_op) = current_candidates_map.get(&op_char) {
//                 for &slot_ptr in slots_for_op {
//                     let raw_node_at_slot: NodeRawPtr = unsafe { &**slot_ptr as NodeRawPtr };
//                     if !slots_already_holding_rot.contains(&slot_ptr) &&
//                        !original_nodes_consumed_as_rot_elements.contains(&raw_node_at_slot) {
//                         available_slots_for_pack.push(slot_ptr);
//                     }
//                 }
//             }

//             if available_slots_for_pack.len() < min_pack_size {
//                 break; 
//             }

//             available_slots_for_pack.shuffle(rng);

//             let current_max_packable = std::cmp::min(max_pack_size, available_slots_for_pack.len());
//             if current_max_packable < min_pack_size { 
//                 break;
//             }
//             let pack_n = rng.gen_range(min_pack_size..=current_max_packable);
            
//             if pack_n == 0 || available_slots_for_pack.len() < pack_n { 
//                 continue;
//             }

//             let slots_for_current_pack: Vec<SlotPtr> = available_slots_for_pack.iter().take(pack_n).cloned().collect();

//             debug!("Attempting to pack {} leaf-operations of type '{}'", pack_n, op_char);

//             let mut snapshot_of_original_nodes_for_rot_elements: Vec<Box<Expr>> = Vec::new();
//             for &slot_ptr_for_snapshot in &slots_for_current_pack {
//                 let original_node_ref: &Expr = unsafe { &**slot_ptr_for_snapshot };
//                 let original_node_raw_ptr: NodeRawPtr = original_node_ref as NodeRawPtr;
                
//                 snapshot_of_original_nodes_for_rot_elements.push(Box::new(original_node_ref.clone()));                
//                 original_nodes_consumed_as_rot_elements.insert(original_node_raw_ptr);
//                 debug!("  Adding to pack (snapshot): {}", original_node_ref);
//             }

//             for (i, &target_slot_to_replace) in slots_for_current_pack.iter().enumerate() {
//                 let mut rot_elements_for_this_rot_instance: Vec<Box<Expr>> = Vec::new();
//                 for original_node_snapshot_box in &snapshot_of_original_nodes_for_rot_elements {
//                     rot_elements_for_this_rot_instance.push(original_node_snapshot_box.clone());
//                 }

//                 let new_rot_node_expr = Expr::Rot {
//                     rotation_index: i,
//                     elements: rot_elements_for_this_rot_instance,
//                 };
                
//                 debug!("  Replacing node {} with Rot{}(...)", unsafe { &**target_slot_to_replace }, i);
//                 unsafe {
//                     *target_slot_to_replace = Box::new(new_rot_node_expr);
//                 }
//                 slots_already_holding_rot.insert(target_slot_to_replace);
//             }
//             made_a_pack_this_op_type = true;

//             if !made_a_pack_this_op_type { 
//                 break;
//             }
//         }
//     }
//     root 
// }

// // **************************************************************************


pub fn create_padding_expr(for_op: char) -> Box<Expr> {
    match for_op {
        '+' => { // Additive identity is 0
            Box::new(Expr::BinOp {
                op: '+', // Use a neutral operation for padding, e.g. 0+0
                left: Box::new(Expr::Var("0".to_string())),
                right: Box::new(Expr::Var("0".to_string())),
            })
            // Alternative if scalar 0 is allowed in Vec: Box::new(Expr::Var("0".to_string()))
        }
        '-' => { // Additive identity is 0
            Box::new(Expr::BinOp {
                op: '-', // Use a neutral operation for padding, e.g. 0-0
                left: Box::new(Expr::Var("0".to_string())),
                right: Box::new(Expr::Var("0".to_string())),
            })
            // Alternative if scalar 0 is allowed in Vec: Box::new(Expr::Var("0".to_string()))
        }
        '*' => { // Multiplicative identity is 1
             Box::new(Expr::BinOp {
                op: '*', // e.g. 1*1
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::Var("1".to_string())),
            })
            // Alternative if scalar 1 is allowed in Vec: Box::new(Expr::Var("1".to_string()))
        }
        _ => { // Default padding for other/unknown ops
            Box::new(Expr::BinOp {
                op: '?', // Placeholder for unknown op context
                left: Box::new(Expr::Var("pad".to_string())),
                right: Box::new(Expr::Var("pad".to_string())),
            })
        }
    }
}


pub fn vectorize_remaining_scalars(
    root_expr: Box<Expr>,
    target_vector_width: usize,
    rng: &mut impl Rng,
) -> Box<Expr> {
    if target_vector_width == 0 {
        debug!("Target vector width is 0, skipping scalar vectorization.");
        return root_expr;
    }
    // Note: padding_expr is now created inside the recursive call based on the op
    vectorize_scalars_recursive(root_expr, target_vector_width, rng)
}

// // Renamed the recursive helper to avoid confusion with the public one
pub fn vectorize_scalars_recursive(
    current_expr_box: Box<Expr>, // Take Box<Expr> by value
    target_vector_width: usize,
    rng: &mut impl Rng,
) -> Box<Expr> {
    match *current_expr_box { // Dereference the Box to match on the Expr
        Expr::Var(_) => current_expr_box, // Variables are returned as is (re-box if needed, but here current_expr_box is already Boxed)
        Expr::Rot { .. } => current_expr_box, // Rot nodes are assumed already vectorized

        Expr::BinOp { op, left, right } => {
            // Recursively process children first
            let new_left = vectorize_scalars_recursive(left, target_vector_width, rng);
            let new_right = vectorize_scalars_recursive(right, target_vector_width, rng);

            // Now check if THIS BinOp (with its potentially transformed children)
            // should itself be vectorized because its *original form* or *current form after child transform*
            // is a scalar operation (Var op Var).
            // The check should be on new_left and new_right.
            if let (Expr::Var(ref _ln), Expr::Var(ref _rn)) = (&*new_left, &*new_right) {
                // It's a BinOp whose children are now simple variables. This is a "scalar" operation.
                // This means it wasn't complex enough to be broken down further,
                // or it was an original Var-Op-Var.
                debug!("Found scalar BinOp to vectorize: ({} {} {})", new_left, op, new_right);

                let padding_expr_for_this_op = create_padding_expr(op);
                let mut elements = Vec::with_capacity(target_vector_width);
                
                let rot_idx_for_this_scalar = if target_vector_width > 0 {
                    rng.gen_range(0..target_vector_width)
                } else { 
                    0 // Should not happen if target_vector_width > 0 is checked at entry
                };

                for i in 0..target_vector_width {
                    if i == rot_idx_for_this_scalar {
                        // Reconstruct the original scalar BinOp to be placed in the vector.
                        // new_left and new_right are already Box<Expr>.
                        elements.push(Box::new(Expr::BinOp {
                            op,
                            left: new_left.clone(), // Clone them as they are used to form this element
                            right: new_right.clone(),
                        }));
                    } else {
                        elements.push(padding_expr_for_this_op.clone());
                    }
                }
                debug!("  Vectorized to Rot with idx {} and {} elements using padding for op '{}'", rot_idx_for_this_scalar, elements.len(), op);
                Box::new(Expr::Rot {
                    rotation_index: rot_idx_for_this_scalar,
                    elements,
                })
            } else {
                
                Box::new(Expr::BinOp { op, left: new_left, right: new_right })
            }
        }
    }
}

// **************************************************************************
// NEW BEAM SEARCH MUTATION FUNCTION
// **************************************************************************

/// Finds a random `Rot` node in an already-packed AST and shuffles its internal
/// `elements` vector. This serves as a "mutation" action for the beam search,
/// allowing it to explore different permutations of an existing packing solution.
pub fn mutate_by_reshuffling_a_pack(mut root: Box<Expr>, rng: &mut impl Rng) -> Box<Expr> {
    // 1. Collect pointers to all mutable `Rot` nodes in the AST.
    let mut rot_node_slots: Vec<*mut Box<Expr>> = Vec::new();
    collect_rot_nodes_recursive(&mut root, &mut rot_node_slots);

    // If there are no Rot nodes to mutate, return the AST as is.
    if rot_node_slots.is_empty() {
        debug!("Mutation skipped: No Rot nodes found to reshuffle.");
        return root;
    }

    // 2. Pick one random Rot node to mutate from the collected list.
    let chosen_slot_ptr = *rot_node_slots.choose(rng).unwrap();

    // 3. Mutate the chosen node by shuffling its `elements` vector in place.
    // This is an unsafe operation because we are directly manipulating the tree
    // via a raw pointer.
    unsafe {
        if let Expr::Rot { elements, .. } = &mut **chosen_slot_ptr {
            debug!("Applying re-shuffle mutation to a Rot node with {} elements.", elements.len());
            elements.shuffle(rng);
        }
    }

    // 4. Return the root of the now-mutated tree.
    root
}

/// A helper function to recursively traverse the AST and collect mutable pointers
/// to every `Rot` node.
fn collect_rot_nodes_recursive(current_node_slot: *mut Box<Expr>, rot_nodes: &mut Vec<*mut Box<Expr>>) {
    // This function uses unsafe pointers to traverse the mutable tree structure.
    unsafe {
        match &mut **current_node_slot {
            Expr::Rot { elements, .. } => {
                // Found a Rot node, add its pointer to our list.
                rot_nodes.push(current_node_slot);
                // Also recurse into the elements, as they could contain more Rot nodes.
                for elem in elements {
                    collect_rot_nodes_recursive(elem, rot_nodes);
                }
            }
            Expr::BinOp { left, right, .. } => {
                // If it's a binary operation, recurse on both children.
                collect_rot_nodes_recursive(left, rot_nodes);
                collect_rot_nodes_recursive(right, rot_nodes);
            }
            Expr::Var(_) => {
                // A variable is a leaf, so we stop recursing down this path.
            }
        }
    }
}

