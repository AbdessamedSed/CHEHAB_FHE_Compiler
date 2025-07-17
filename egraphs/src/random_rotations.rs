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
                
                let child_prefix = prefix + (if is_last { "    " } else { "│   " });
                if !elements.is_empty() {
                    debug!("{}  └── Elements List (for above Rot):", child_prefix);
                    let element_list_base_prefix = child_prefix.clone() + "      ";
                    
                    for (i, elem) in elements.iter().enumerate() {
                        let is_last_element = i == elements.len() - 1;
                        let element_prefix_arm = if is_last_element { "└── " } else { "├── " };
                        
                        let selected_info = if i == *rotation_index {
                            " (conceptually selected by this Rot)"
                        } else {
                            ""
                        };
                        debug!("{}{}[{}]:{}", element_list_base_prefix, element_prefix_arm, i, selected_info);
                        
                        let sub_element_prefix = element_list_base_prefix.clone() + (if is_last_element { "    " } else { "│   " });
                        elem.print_tree(indent + 3, true, sub_element_prefix);
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
            Some(c) if c.is_alphanumeric() || c == '_' => {
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

    if !visited_raw_nodes.insert(current_node_raw) {
        return;
    }

    if let Expr::BinOp { op, ref mut left, ref mut right } = **current_node_box {
        let left_is_var = matches!(**left, Expr::Var(_));
        let right_is_var = matches!(**right, Expr::Var(_));

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
    
    let root_slot_ptr: SlotPtr = &mut root as *mut Box<Expr>;
    let op_types_to_process: Vec<char> = {
        let mut initial_op_types_map = HashMap::new();
        let mut temp_visited = HashSet::new();
        collect_packable_leaf_operations_recursive(root_slot_ptr, &mut initial_op_types_map, &mut temp_visited);
        initial_op_types_map.keys().cloned().collect()
    };

    for &op_char in &op_types_to_process {
        let mut slots_already_holding_rot = HashSet::new();
        let mut original_nodes_consumed = HashSet::new();
        loop {
            let mut current_candidates_map = HashMap::new();
            let mut visited_this_pass = HashSet::new();
            collect_packable_leaf_operations_recursive(root_slot_ptr, &mut current_candidates_map, &mut visited_this_pass);
            
            let mut available_slots: Vec<SlotPtr> = current_candidates_map.get(&op_char)
                .map_or(Vec::new(), |slots| {
                    slots.iter().filter(|&&slot| {
                        let raw_node_at_slot: NodeRawPtr = unsafe { &**slot };
                        !slots_already_holding_rot.contains(&slot) && !original_nodes_consumed.contains(&raw_node_at_slot)
                    }).cloned().collect()
                });

            if available_slots.len() < min_pack_size { break; }

            available_slots.shuffle(rng);
            let pack_n = rng.gen_range(min_pack_size..=std::cmp::min(max_pack_size, available_slots.len()));
            if pack_n == 0 { continue; }

            let slots_for_current_pack: Vec<SlotPtr> = available_slots.iter().take(pack_n).cloned().collect();
            let mut snapshot_of_elements: Vec<Box<Expr>> = Vec::new();
            for &slot_ptr in &slots_for_current_pack {
                let original_node_ref: &Expr = unsafe { &**slot_ptr };
                snapshot_of_elements.push(Box::new(original_node_ref.clone()));
                original_nodes_consumed.insert(original_node_ref as NodeRawPtr);
            }

            for (i, &target_slot) in slots_for_current_pack.iter().enumerate() {
                let new_rot_node = Expr::Rot {
                    rotation_index: i,
                    elements: snapshot_of_elements.clone(),
                };
                unsafe { *target_slot = Box::new(new_rot_node); }
                slots_already_holding_rot.insert(target_slot);
            }
        }
    }
    root 
}

// **************************************************************************
// FIXED AND ROBUST FUNCTIONS
// **************************************************************************

/// **FIXED VERSION**
/// Expands `expr * N` into a sum tree. Crucially, `expr * 0` now correctly
/// returns the ADDITIVE identity element `(+ 0 0)`, not a bare `Var("0")`.
pub fn expand_multiplication_by_integer(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Var(_) => expr,
        Expr::Rot { rotation_index, elements } => {
            Box::new(Expr::Rot {
                rotation_index,
                elements: elements.into_iter().map(expand_multiplication_by_integer).collect(),
            })
        }
        Expr::BinOp { op, left, right } => {
            let new_left = expand_multiplication_by_integer(left);
            let new_right = expand_multiplication_by_integer(right);

            if op == '*' {
                let mut num_opt: Option<u32> = None;
                let mut expr_to_expand_opt: Option<Box<Expr>> = None;

                if let Expr::Var(s) = &*new_right {
                    if let Ok(n) = s.parse::<u32>() {
                        num_opt = Some(n);
                        expr_to_expand_opt = Some(new_left.clone());
                    }
                } else if let Expr::Var(s) = &*new_left {
                    if let Ok(n) = s.parse::<u32>() {
                        num_opt = Some(n);
                        expr_to_expand_opt = Some(new_right.clone());
                    }
                }

                if let (Some(n), Some(the_expr)) = (num_opt, expr_to_expand_opt) {
                    debug!("Expanding multiplication: ({}) * {}", the_expr, n);
                    return match n {
                        // *** FIX IS HERE ***
                        // Previously returned `Var("0")`. Now returns the proper additive identity element.
                        // This prevents raw `0`s from appearing in packed vectors.
                        0 => Box::new(Expr::BinOp {
                            op: '+',
                            left: Box::new(Expr::Var("0".to_string())),
                            right: Box::new(Expr::Var("0".to_string())),
                        }),
                        1 => the_expr,
                        _ => {
                            let mut sum_tree = the_expr.clone();
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
            Box::new(Expr::BinOp { op, left: new_left, right: new_right })
        }
    }
}

/// **FIXED VERSION**
/// Creates a padding expression based on the identity of the operation.
/// The bug where multiplication used `0` instead of `1` is corrected.
pub fn create_padding_expr(for_op: char) -> Box<Expr> {
    match for_op {
        '+' | '-' => { // Additive identity is 0
            Box::new(Expr::BinOp {
                op: '+', // Use addition for the identity element `0+0`
                left: Box::new(Expr::Var("0".to_string())),
                right: Box::new(Expr::Var("0".to_string())),
            })
        }
        '*' => { // Multiplicative identity is 1
             // *** FIX IS HERE ***
             // Previously used `Var("0")`. The correct multiplicative identity is 1.
             Box::new(Expr::BinOp {
                op: '*',
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::Var("1".to_string())),
            })
        }
        _ => { // Default padding for other/unknown ops
            Box::new(Expr::BinOp {
                op: '?',
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
    vectorize_scalars_recursive(root_expr, target_vector_width, rng)
}

pub fn vectorize_scalars_recursive(
    current_expr_box: Box<Expr>,
    target_vector_width: usize,
    rng: &mut impl Rng,
) -> Box<Expr> {
    match *current_expr_box {
        Expr::Var(_) => current_expr_box,
        Expr::Rot { .. } => current_expr_box,
        Expr::BinOp { op, left, right } => {
            let new_left = vectorize_scalars_recursive(left, target_vector_width, rng);
            let new_right = vectorize_scalars_recursive(right, target_vector_width, rng);

            if let (Expr::Var(_), Expr::Var(_)) = (&*new_left, &*new_right) {
                debug!("Found scalar BinOp to vectorize: ({} {} {})", new_left, op, new_right);
                let padding_expr_for_this_op = create_padding_expr(op);
                let mut elements = Vec::with_capacity(target_vector_width);
                let rot_idx_for_this_scalar = if target_vector_width > 0 {
                    rng.gen_range(0..target_vector_width)
                } else { 0 };

                for i in 0..target_vector_width {
                    if i == rot_idx_for_this_scalar {
                        elements.push(Box::new(Expr::BinOp {
                            op,
                            left: new_left.clone(),
                            right: new_right.clone(),
                        }));
                    } else {
                        elements.push(padding_expr_for_this_op.clone());
                    }
                }
                debug!("Vectorized to Rot with idx {} and {} elements using padding for op '{}'", rot_idx_for_this_scalar, elements.len(), op);
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
// BEAM SEARCH MUTATION FUNCTION
// **************************************************************************

/// Finds a random `Rot` node in an already-packed AST and shuffles its internal
/// `elements` vector.
pub fn mutate_by_reshuffling_a_pack(mut root: Box<Expr>, rng: &mut impl Rng) -> Box<Expr> {
    let mut rot_node_slots: Vec<*mut Box<Expr>> = Vec::new();
    collect_rot_nodes_recursive(&mut root, &mut rot_node_slots);

    if rot_node_slots.is_empty() {
        debug!("Mutation skipped: No Rot nodes found to reshuffle.");
        return root;
    }

    let chosen_slot_ptr = *rot_node_slots.choose(rng).unwrap();

    unsafe {
        if let Expr::Rot { elements, .. } = &mut **chosen_slot_ptr {
            debug!("Applying re-shuffle mutation to a Rot node with {} elements.", elements.len());
            elements.shuffle(rng);
        }
    }
    root
}

/// A helper function to recursively traverse the AST and collect mutable pointers
/// to every `Rot` node.
fn collect_rot_nodes_recursive(current_node_slot: *mut Box<Expr>, rot_nodes: &mut Vec<*mut Box<Expr>>) {
    unsafe {
        match &mut **current_node_slot {
            Expr::Rot { elements, .. } => {
                rot_nodes.push(current_node_slot);
                for elem in elements {
                    collect_rot_nodes_recursive(elem, rot_nodes);
                }
            }
            Expr::BinOp { left, right, .. } => {
                collect_rot_nodes_recursive(left, rot_nodes);
                collect_rot_nodes_recursive(right, rot_nodes);
            }
            Expr::Var(_) => {}
        }
    }
}