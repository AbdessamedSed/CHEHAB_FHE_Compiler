use egg::*;
use crate::{
    veclang::{ConstantFold, VecLang},
};
use egg::rewrite as rw;


pub fn is_vec(var1: &'static str,var2: &'static str,var3: &'static str,var4: &'static str) -> impl Fn(&mut EGraph<VecLang, ConstantFold>, Id, &Subst) -> bool {
    let var1_str = var1.parse().unwrap();
    let var2_str = var2.parse().unwrap();
    let var3_str = var3.parse().unwrap();
    let var4_str = var4.parse().unwrap();
    move |egraph : &mut EGraph<VecLang, ConstantFold>, _, subst| {
        let nodes1 = &egraph[subst[var1_str]].nodes ;
        let nodes2 = &egraph[subst[var2_str]].nodes ;
        let nodes3 = &egraph[subst[var3_str]].nodes ;
        let nodes4 = &egraph[subst[var4_str]].nodes ;
        let is_vector1 = nodes1.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector2 = nodes2.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector3 = nodes3.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector4 = nodes4.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        is_vector1&&is_vector2&&is_vector3&&is_vector4
    }
}

/******************************************************************************************/

fn is_vec_mul(var1: &'static str,var2: &'static str,var3: &'static str,var4: &'static str,var5: &'static str,var6: &'static str,var7: &'static str,var8: &'static str) -> impl Fn(&mut EGraph<VecLang, ConstantFold>, Id, &Subst) -> bool {
    let var1_str = var1.parse().unwrap();
    let var2_str = var2.parse().unwrap();
    let var3_str = var3.parse().unwrap();
    let var4_str = var4.parse().unwrap();
    let var5_str = var5.parse().unwrap();
    let var6_str = var6.parse().unwrap();
    let var7_str = var7.parse().unwrap();
    let var8_str = var8.parse().unwrap();
    move |egraph : &mut EGraph<VecLang, ConstantFold>, _, subst| {
        let nodes1 = &egraph[subst[var1_str]].nodes ;
        let nodes2 = &egraph[subst[var2_str]].nodes ;
        let nodes3 = &egraph[subst[var3_str]].nodes ;
        let nodes4 = &egraph[subst[var4_str]].nodes ;
        let nodes5 = &egraph[subst[var5_str]].nodes ;
        let nodes6 = &egraph[subst[var6_str]].nodes ;
        let nodes7 = &egraph[subst[var7_str]].nodes ;
        let nodes8 = &egraph[subst[var8_str]].nodes ;
        let is_vector1 = nodes1.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector2 = nodes2.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector3 = nodes3.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector4 = nodes4.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector5 = nodes5.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector6 = nodes6.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector7 = nodes7.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));
        let is_vector8 = nodes8.iter().all(|n| matches!(n, VecLang::Vec(_) | VecLang::VecAdd(_) | VecLang::VecMul(_) | VecLang::VecMinus(_) | VecLang::VecNeg(_)));

        is_vector1&&is_vector2&&is_vector3&&is_vector4&&is_vector5&&is_vector6&&is_vector7&&is_vector8
    }
}

/******************************************************************************************/

pub fn vector_assoc_add_rules() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("assoc-balan-add-1"; 
            "(VecAdd ?x (VecAdd ?y (VecAdd ?z ?t)))" => 
            "(VecAdd (VecAdd ?x ?y) (VecAdd ?z ?t))"
            if is_vec("?x","?y","?z","?t")
        ),
        rw!("assoc-balan-add-2"; 
            "(VecAdd ?x (VecAdd (VecAdd ?z ?t) ?y))" => 
            "(VecAdd (VecAdd ?x ?z) (VecAdd ?t ?y))"
            if is_vec("?x","?z","?t","?y")
        ),
        rw!("assoc-balan-add-3"; 
            "(VecAdd (VecAdd (VecAdd ?x ?y) ?z) ?t)" => 
            "(VecAdd (VecAdd ?x ?y) (VecAdd ?z ?t))"
            if is_vec("?x","?z","?t","?y")
        ),
        rw!("assoc-balan-add-4"; 
            "(VecAdd (VecAdd ?x (VecAdd ?y ?z)) ?t)" => 
            "(VecAdd (VecAdd ?x ?y) (VecAdd ?z ?t))"
            if is_vec("?x","?z","?t","?y")
        ),
    ];
    rules 
}

/*******************************************************************************************/

pub fn vector_assoc_min_rules() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules : Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("assoc-balan-sub-1"; 
            "(VecMinus (VecMinus (VecMinus ?x ?y) ?z) ?t)" => 
            "(VecMinus (VecMinus ?x ?y) (VecAdd ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-sub-2"; 
            "(VecMinus (VecMinus ?z (VecMinus ?x ?y)) ?t)" => 
            "(VecAdd (VecMinus ?z ?x) (VecMinus ?y ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-sub-3"; 
            "(VecMinus ?x (VecMinus ?y (VecMinus ?z ?t)))" => 
            "(VecMinus (VecMinus ?x ?y) (VecAdd ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-sub-4"; 
            "(VecMinus ?x (VecMinus (VecMinus ?z ?t) ?y))" => 
            "(VecAdd (VecMinus ?x ?z) (VecAdd ?t ?y))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("simplify-sub-1"; 
            "(VecMinus (VecAdd ?x ?y) ?x)" => 
            "?y" 
            if is_vec("?x", "?y", "?x", "?y")
        ),
        rw!("simplify-sub-2"; 
            "(VecMinus (VecAdd ?x ?y) ?y)" =>
            "?x" 
            if is_vec("?x", "?y", "?x", "?y")
        ),
        rw!("simplify-sub-3"; 
            "(VecMinus ?x (VecAdd ?x ?y))" => 
            "(VecNeg ?y)" 
            if is_vec("?x", "?y", "?x", "?y")
        ),
        rw!("simplify-sub-4"; 
            "(VecMinus ?y (VecAdd ?x ?y))" => 
            "(VecNeg ?x)" 
            if is_vec("?x", "?y", "?x", "?y")
        ),
        rw!("simplify-sub-5"; 
            "(VecMinus (VecMinus ?x ?y) ?x)" => 
            "(VecNeg ?y)" 
            if is_vec("?x", "?y", "?x", "?y")
        ),
    ];
    rules
}

/******************************************************************************************/

pub fn vector_assoc_add_mul_rules() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("assoc-balan-add-mul-1"; 
            "(VecAdd (VecAdd (VecAdd (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)) (VecMul ?b1 ?b2)) (VecMul ?a1 ?a2))" => 
            "(VecAdd (VecAdd (VecMul ?a1 ?a2) (VecMul ?b1 ?b2)) (VecAdd (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rw!("assoc-balan-add-mul-2"; 
            "(VecAdd (VecMul ?a1 ?a2) (VecAdd (VecMul ?b1 ?b2) (VecAdd (VecMul ?c1 ?c2) (VecMul ?d1 ?d2))))" => 
            "(VecAdd (VecAdd (VecMul ?a1 ?a2) (VecMul ?b1 ?b2)) (VecAdd (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rw!("assoc-balan-add-mul-3"; 
            "(VecAdd (VecAdd (VecMul ?a1 ?a2) (VecAdd (VecMul ?b1 ?b2) (VecMul ?c1 ?c2))) (VecMul ?d1 ?d2))" => 
            "(VecAdd (VecAdd (VecMul ?a1 ?a2) (VecMul ?b1 ?b2)) (VecAdd (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rw!("assoc-balan-add-mul-4"; 
            "(VecAdd (VecAdd (VecMul ?a ?b) ?c) ?d)" => 
            "(VecAdd (VecMul ?a ?b) (VecAdd ?c ?d))"
            if is_vec("?a","?b","?c","?c")
        ),
        rw!("assoc-balan-add-mul-5"; 
            "(VecAdd ?a (VecAdd ?b (VecMul ?c ?d)))" =>
            "(VecAdd (VecAdd ?a ?b) (VecMul ?c ?d))"
            if is_vec("?a","?b","?c","?d")
        ),
        rw!("distribute-mul-over-add-1"; 
            "(VecMul ?a (VecAdd ?b ?c))" => 
            "(VecAdd (VecMul ?a ?b) (VecMul ?a ?c))"
            if is_vec("?a","?b","?c","?c")
        ),
        // rw!("part-fold-dist-add-1-4";
        //     "(VecMul (VecAdd (VecMul ?ct_x ?c0) ?c1) (VecAdd (VecMul ?ct_x ?c2) ?c3))" =>
        //     "(VecAdd (VecMul ?ct_x (VecAdd (VecMul ?c0 ?c2)) (VecAdd (VecMul ?c0 ?c3) (VecMul ?c1 ?c2))) )"
        //     if is_vec("")
        // ),
        // rw!("part-fold-dist-add-1-4"; 
        //     "(VecMul (VecAdd (VecMul ?ct_x ?c0) ?c1) (VecAdd (VecMul ?ct_x ?c2) ?c3))" => 
        //     "(VecAdd (VecMul ?ct_x (VecAdd (VecMul ?ct_x (VecMul ?c0 ?c2)) (VecAdd (VecMul ?c0 ?c3) (VecMul ?c1 ?c2)))) (VecMul ?c1 ?c3))"
        //     if is_vec("?c0", "?c1", "?c2", "?c3")
        // ),
        rw!("part-fold-dist-add-2";
            "(VecMul (VecAdd ?ct_x ?c0) ?c1)" =>
            "(VecAdd (VecMul ?ct_x ?c1) (VecMul ?c0 ?c1))"
            if is_vec("?ct_x", "?c0", "?c1", "?c1")
        )

    ];
    rules
}

/******************************************************************************************/

pub fn vector_assoc_add_min_rules() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("assoc-balan-add-min-1"; 
            "(VecAdd (VecAdd (VecAdd (VecMinus ?c1 ?c2) (VecMinus ?d1 ?d2)) (VecMinus ?b1 ?b2)) (VecMinus ?a1 ?a2))" => 
            "(VecAdd (VecAdd (VecMinus ?a1 ?a2) (VecMinus ?b1 ?b2)) (VecAdd (VecMinus ?c1 ?c2) (VecMinus ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rw!("assoc-balan-add-min-2"; 
            "(VecAdd (VecMinus ?a1 ?a2) (VecAdd (VecMinus ?b1 ?b2) (VecAdd (VecMinus ?c1 ?c2) (VecMinus ?d1 ?d2))))" => 
            "(VecAdd (VecAdd (VecMinus ?a1 ?a2) (VecMinus ?b1 ?b2)) (VecAdd (VecMinus ?c1 ?c2) (VecMinus ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rw!("assoc-balan-add-min-3"; 
            "(VecAdd (VecAdd (VecMinus ?a1 ?a2) (VecAdd (VecMinus ?b1 ?b2) (VecMinus ?c1 ?c2))) (VecMinus ?d1 ?d2))" => 
            "(VecAdd (VecAdd (VecMinus ?a1 ?a2) (VecMinus ?b1 ?b2)) (VecAdd (VecMinus ?c1 ?c2) (VecMinus ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rw!("assoc-balan-add-4"; 
            "(VecAdd ?x (VecAdd (VecAdd ?z ?t) ?y))" => 
            "(VecAdd (VecAdd ?x ?z) (VecAdd ?t ?y))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-1"; 
            "(VecAdd (VecMinus (VecAdd ?x ?y) ?z) ?t)" => 
            "(VecMinus (VecAdd ?x ?y) (VecMinus ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-2"; 
            "(VecAdd (VecMinus ?z (VecAdd ?x ?y)) ?t)" => 
            "(VecMinus (VecMinus ?z ?x) (VecMinus ?y ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-3"; 
            "(VecAdd (VecAdd (VecMinus ?x ?y) ?z) ?t)" => 
            "(VecAdd (VecMinus ?x ?y) (VecAdd ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-4"; 
            "(VecAdd (VecAdd ?z (VecMinus ?x ?y)) ?t)" => 
            "(VecMinus (VecAdd ?z ?x) (VecMinus ?y ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-5"; 
            "(VecAdd (VecMinus (VecMinus ?x ?y) ?z) ?t)" =>
            "(VecMinus (VecMinus ?x ?y) (VecMinus ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-6"; 
            "(VecAdd (VecMinus ?z (VecMinus ?x ?y)) ?t)" => 
            "(VecAdd (VecMinus ?z ?x) (VecAdd ?y ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-7"; 
            "(VecAdd ?x (VecMinus ?y (VecAdd ?z ?t)))" => 
            "(VecMinus (VecAdd ?x ?y) (VecAdd ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-8"; 
            "(VecAdd ?x (VecMinus (VecAdd ?z ?t) ?y))" => 
            "(VecAdd (VecAdd ?x ?z) (VecMinus ?t ?y))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-9"; 
            "(VecAdd ?x (VecAdd ?y (VecMinus ?z ?t)))" => 
            "(VecAdd (VecAdd ?x ?y) (VecMinus ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-10"; 
            "(VecAdd ?x (VecAdd (VecMinus ?z ?t) ?y))" => 
            "(VecMinus (VecAdd ?x ?z) (VecMinus ?t ?y))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-11"; 
            "(VecAdd ?x (VecMinus ?y (VecMinus ?z ?t)))" => 
            "(VecMinus (VecAdd ?x ?y) (VecMinus ?z ?t))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-add-sub-12"; 
            "(VecAdd ?x (VecMinus (VecMinus ?z ?t) ?y))" => 
            "(VecMinus (VecAdd ?x ?z) (VecAdd ?t ?y))" 
            if is_vec("?x", "?y", "?z", "?t")
        ),
    ];
    rules
}


/*****************************************************************************************/

pub fn vector_assoc_min_mul_rules() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rewrite!("assoc-balan-min-mul-1"; 
            "(VecMinus (VecMinus (VecMinus (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)) (VecMul ?b1 ?b2)) (VecMul ?a1 ?a2))" => 
            "(VecMinus (VecMinus (VecMul ?a1 ?a2) (VecMul ?b1 ?b2)) (VecAdd (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rewrite!("assoc-balan-min-mul-2"; 
            "(VecMinus (VecMul ?a1 ?a2) (VecMinus (VecMul ?b1 ?b2) (VecMinus (VecMul ?c1 ?c2) (VecMul ?d1 ?d2))))" => 
            "(VecMinus (VecMinus (VecMul ?a1 ?a2) (VecMul ?b1 ?b2)) (VecMinus (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
        rewrite!("assoc-balan-min-mul-3"; 
            "(VecMinus (VecMinus (VecMul ?a1 ?a2) (VecMinus (VecMul ?b1 ?b2) (VecMul ?c1 ?c2))) (VecMul ?d1 ?d2))" => 
            "(VecMinus (VecMinus (VecMul ?a1 ?a2) (VecMul ?b1 ?b2)) (VecMinus (VecMul ?c1 ?c2) (VecMul ?d1 ?d2)))"
            if is_vec_mul("?a1","?a2","?b1","?b2","?c1","?c2","?d1","?d2")
        ),
    ];
    rules
}

/***************************************************************/

pub fn simplificatio_rules_add_min() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rewrite!("simplify-add-negate-1-1";
            "(VecAdd ?x (VecNeg ?y))" => 
            "(VecMinus ?x ?y)"
            if is_vec("?x","?y", "?x", "?y")
        ),
        rewrite!("simplify-add-negate-1-2";
            "(VecAdd (VecNeg ?y) ?x)" => 
            "(VecMinus ?x ?y)"
            if is_vec("?x","?y", "?x", "?y")
        ),
        rewrite!("simplify-add-sub-1-1";
            "(VecAdd ?x (VecMinus ?y ?x))" => 
            "(?x)"
            if is_vec("?x","?y", "?x", "?y")
        ),
        rewrite!("simplify-add-sub-1-2";
            "(VecAdd (VecMinus ?y ?x) ?x)" => 
            "(?x)"
            if is_vec("?x","?y", "?x", "?y")
        ),
        rewrite!("simplify-add-negate-2-1";
            "(VecAdd ?x (VecMinus (VecNeg ?y) ?z))" =>
            "(VecMinus ?x (VecAdd ?y ?z))"
            if is_vec("?x","?y", "?z", "?x")
        ),
        rewrite!("simplify-add-negate-2-2";
            "(VecAdd (VecMinus (VecNeg ?y) ?z) ?x)" => 
            "(VecMinus ?x (VecAdd ?y ?z))"
            if is_vec("?x","?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-2-1";
            "(VecAdd (VecAdd (VecMinus ?x ?y) ?z) ?y)" => 
            "(VecAdd ?x ?z)"
            if is_vec("?x", "?y", "?z", "?x")    
        ),
        rewrite!("simplify-add-sub-2-2";
            "(VecAdd (VecAdd ?z (VecMinus ?x ?y)) ?y)" => 
            "(VecAdd ?z ?x)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-2-3";
            "(VecAdd ?x (VecAdd (VecMinus ?y ?x) ?z))" => 
            "(VecAdd ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-2-4";
            "(VecAdd ?x (VecAdd ?z (VecMinus ?y ?x)))" => 
            "(VecAdd ?z ?y)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-3-1";
            "(VecAdd (VecMinus ?x ?y) (VecMinus ?z ?y))" => 
            "(VecMinus ?x ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-3-2";
            "(VecAdd (VecMinus ?x ?y) (VecMinus ?z ?x))" => 
            "(VecMinus ?z ?y)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-3-3";
            "(VecAdd (VecMinus ?x ?y) (VecAdd ?y ?z))" => 
            "(VecAdd ?z ?x)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-3-4";
            "(VecAdd (VecMinus ?x ?y) (VecAdd ?z ?y))" => 
            "(VecAdd ?x ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-4-1";
            "(VecAdd ?x (VecMinus (VecMinus ?y ?x) ?z))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-4-2";
            "(VecAdd (VecMinus (VecMinus ?x ?y) ?z) ?y)" => 
            "(VecMinus ?x ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-4-3";
            "(VecAdd ?x (VecMinus ?y (VecAdd ?x ?z)))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-4-4";
            "(VecAdd ?x (VecMinus ?y (VecAdd ?z ?x)))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-4-5";
            "(VecAdd (VecMinus ?x (VecAdd ?y ?z)) ?y)" => 
            "(VecMinus ?x ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-add-sub-4-6";
            "(VecAdd (VecMinus ?x (VecAdd ?y ?z)) ?z)" => 
            "(VecMinus ?x ?y)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("part-fold-zero_m-1";
            "(VecAdd ?x (VecMinus (VecNeg ?y) ?z))" => 
            "(VecMinus ?x (VecAdd ?y ?z))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("part-fold-zero_m-2";
            "(VecAdd (VecMinus (VecNeg ?x) ?y) ?z)" => 
            "(VecMinus ?z (VecAdd ?x ?y))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("part-fold-assoc-add-sub-3";
            "(VecAdd (VecMinus ?c0 ?x) ?c1)" =>
            "(VecMinus (VecAdd ?c0 ?c1) ?x)"
            if is_vec("?c0", "?c1", "?x", "?c0")
        ),
        rw!("sub_0"; 
            "(VecMinus ?x 0)" => 
            "?x" 
            if is_vec("?x", "?x", "?x", "?x")
        ),
        rw!("sub_self"; 
            "(VecMinus ?x ?x)" =>
            "0" 
            if is_vec("?x", "?x", "?x", "?x")
        ),
        rw!("simplify-sub-negate"; 
            "(VecMinus ?x (VecNeg ?y))" => 
            "(VecAdd ?x ?y)" 
            if is_vec("?x", "?y", "?x", "?y")
        ),
        rw!("simplify-sub-mul_negate-1"; 
            "(VecMinus ?z (VecMul ?x (VecNeg ?y)))" => 
            "(VecAdd ?z (VecMul ?x ?y))" 
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rw!("simplify-sub-mul_negate-2"; 
            "(VecMinus ?z (VecMul (VecNeg ?y) ?x))" => 
            "(VecAdd ?z (VecMul ?y ?x))" 
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-2-1"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd ?x ?z))" =>
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?x", "?z")
        ),
        rewrite!("simplify-sub-2-2"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd ?z ?x))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-2-3"; 
            "(VecMinus (VecAdd ?y ?x) (VecAdd ?x ?z))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?y", "?x", "?x", "?z")
        ),
        rewrite!("simplify-sub-2-4"; 
            "(VecMinus (VecAdd ?y ?x) (VecAdd ?z ?x))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?y", "?x", "?z", "?x")
        ),
        rewrite!("simplify-sub-3-1"; 
            "(VecMinus (VecAdd (VecAdd ?x ?y) ?z) ?x)" => 
            "(VecAdd ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-3-2"; 
            "(VecMinus (VecAdd (VecAdd ?y ?x) ?z) ?x)" => 
            "(VecAdd ?y ?z)"
            if is_vec("?y", "?x", "?z", "?x")
        ),
        rewrite!("simplify-sub-3-3"; 
            "(VecMinus (VecAdd ?z (VecAdd ?x ?y)) ?x)" => 
            "(VecAdd ?z ?y)"
            if is_vec("?z", "?x", "?y", "?x")
        ),
        rewrite!("simplify-sub-3-4"; 
            "(VecMinus (VecAdd ?z (VecAdd ?y ?x)) ?x)" => 
            "(VecAdd ?z ?y)"
            if is_vec("?z", "?y", "?x", "?x")
        ),
        rewrite!("simplify-sub-4-1"; 
            "(VecMinus ?x (VecAdd ?y (VecMinus ?x ?z)))" => 
            "(VecMinus ?z ?y)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-4-2"; 
            "(VecMinus ?x (VecAdd (VecMinus ?x ?y) ?z))" => 
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-4-3"; 
            "(VecMinus (VecAdd ?x (VecMinus ?y ?z)) ?y)" => 
            "(VecMinus ?x ?z)"
            if is_vec("?x", "?y", "?z", "?y")
        ),
        rewrite!("simplify-sub-4-4"; 
            "(VecMinus (VecAdd (VecMinus ?x ?y) ?z) ?x)" => 
            "(VecMinus ?z ?y)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-5-1"; 
            "(VecMinus ?x (VecAdd ?y (VecAdd ?x ?z)))" => 
            "(VecNeg (VecAdd ?y ?z))"
            if is_vec("?x", "?y", "?x", "?z")
        ),
        rewrite!("simplify-sub-5-2"; 
            "(VecMinus ?x (VecAdd ?y (VecAdd ?z ?x)))" => 
            "(VecNeg (VecAdd ?y ?z))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-5-3"; 
            "(VecMinus ?x (VecAdd (VecAdd ?x ?y) ?z))" => 
            "(VecNeg (VecAdd ?y ?z))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-5-4"; 
            "(VecMinus ?x (VecAdd (VecAdd ?y ?x) ?z))" =>
            "(VecNeg (VecAdd ?y ?z))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-6-1-0"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd ?z (VecAdd ?w ?x)))" => 
            "(VecMinus ?y (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-2-0"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd ?z (VecAdd ?w ?y)))" => 
            "(VecMinus ?x (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-3"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd ?z (VecAdd ?x ?w)))" => 
            "(VecMinus ?y (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-4"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd (VecAdd ?x ?z) ?w))" => 
            "(VecMinus ?y (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-5"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd (VecAdd ?x ?z) ?w))" => 
            "(VecMinus ?y (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-6"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd (VecAdd ?y ?z) ?w))" => 
            "(VecMinus ?x (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-7"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd (VecAdd ?z ?x) ?w))" => 
            "(VecMinus ?y (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-8"; 
            "(VecMinus (VecAdd ?x ?y) (VecAdd (VecAdd ?z ?y) ?w))" => 
            "(VecMinus ?x (VecAdd ?z ?w))"
            if is_vec("?x", "?y", "?z", "?w")
        ),
        rewrite!("simplify-sub-6-1-1"; 
            "(VecMinus (VecMinus ?x ?y) (VecAdd ?x ?z))" => 
            "(VecMinus (VecNeg ?y) ?z)"
            if is_vec("?x", "?y", "?x", "?z")
        ),
        rewrite!("simplify-sub-6-2-1"; 
            "(VecMinus (VecMinus ?x ?y) (VecAdd ?z ?x))" => 
            "(VecMinus (VecNeg ?y) ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-7-1"; 
            "(VecMinus (VecMinus (VecAdd ?x ?y) ?z) ?x)" => 
            "(VecMinus ?y ?z)"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-7-2"; 
            "(VecMinus (VecMinus (VecAdd ?x ?y) ?z) ?y)" => 
            "(VecMinus ?x ?z)"
            if is_vec("?x", "?y", "?z", "?y")
        ),
        rewrite!("simplify-sub-8-1"; 
            "(VecNeg (VecAdd ?x (VecMinus ?y ?z)))" => 
            "(VecMinus ?z (VecAdd ?x ?y))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-8-2"; 
            "(VecNeg (VecAdd (VecMinus ?x ?y) ?z))" => 
            "(VecMinus ?y (VecAdd ?x ?z))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("simplify-sub-9"; 
            "(VecMinus (VecMinus (VecMinus ?x ?y) ?z) ?x)" => 
            "(VecNeg (VecAdd ?y ?z))"
            if is_vec("?x", "?y", "?z", "?x")
        ),
        rewrite!("fold-negate";
            "(VecNeg (VecNeg ?x))" =>
            "?x"
            if is_vec("?x", "?x", "?x", "?x")
        ),
        rewrite!("fold-negate-sub";
            "(VecNeg (VecMinus ?x ?y))" =>
            "(VecMinus ?y ?x)"
            if is_vec("?x", "?y", "?x", "?y")
        ),
        rewrite!("flod-negate-add";
            "(VecNeg (VecMinus (VecNeg ?x) ?y))" =>
            "(VecAdd ?x ?y)"
            if is_vec("?x", "?y", "?x", "?y")
        ),
    ];
    rules
} 

/*************************************************************/

pub fn simplificatio_rules_add_mul() -> Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("fact-add-1-1-1"; 
            "(VecAdd (VecMul ?x ?ct_y) (VecMul ?z ?ct_y))" => 
            "(VecMul (VecAdd ?x ?z) ?ct_y)"
            if is_vec("?x", "?z", "?ct_y", "?x")
        ),

        rw!("fact-add-1-2-1"; 
            "(VecAdd (VecMul ?ct_x ?pt_y) (VecMul ?ct_z ?pt_y))" => 
            "(VecMul (VecAdd ?ct_x ?ct_z) ?pt_y)"
            if is_vec("?ct_x", "?ct_z", "?pt_y", "?ct_x")
        ),

        rw!("fact-add-2-1-1"; 
            "(VecAdd (VecMul ?x ?ct_y) (VecMul ?ct_y ?z))" => 
            "(VecMul (VecAdd ?x ?z) ?ct_y)"
            if is_vec("?x", "?z", "?ct_y", "?x")
        ),

        rw!("fact-add-2-2-1"; 
            "(VecAdd (VecMul ?ct_x ?pt_y) (VecMul ?pt_y ?ct_z))" =>
            "(VecMul (VecAdd ?ct_x ?ct_z) ?pt_y)"
            if is_vec("?ct_x", "?ct_z", "?pt_y", "?ct_x")
        ),

        rw!("fact-add-3-1-1"; 
            "(VecAdd (VecMul ?ct_y ?x) (VecMul ?z ?ct_y))" => 
            "(VecMul ?ct_y (VecAdd ?x ?z))"
            if is_vec("?ct_y", "?x", "?z", "?ct_y")
        ),

        rw!("fact-add-3-2-1"; 
            "(VecAdd (VecMul ?pt_y ?ct_x) (VecMul ?ct_z ?pt_y))" => 
            "(VecMul ?pt_y (VecAdd ?ct_x ?ct_z))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?pt_y")
        ),

        rw!("fact-add-4-1-1"; 
            "(VecAdd (VecMul ?ct_y ?x) (VecMul ?ct_y ?z))" => 
            "(VecMul ?ct_y (VecAdd ?x ?z))"
            if is_vec("?ct_y", "?x", "?z", "?ct_y")
        ),

        rw!("fact-add-4-2-1"; 
            "(VecAdd (VecMul ?pt_y ?ct_x) (VecMul ?pt_y ?ct_z))" => 
            "(VecMul ?pt_y (VecAdd ?ct_x ?ct_z))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?pt_y")
        ),

        rw!("fact_one-add-1"; 
            "(VecAdd ?x (VecMul ?x ?y))" => 
            "(VecMul ?x (VecAdd ?y 1))"
            if is_vec("?x", "?y", "?x", "?x")
        ),

        rw!("fact_one-add-2"; 
            "(VecAdd ?x (VecMul ?y ?x))" => 
            "(VecMul (VecAdd ?y 1) ?x)"
            if is_vec("?x", "?y", "?x", "?x")
        ),

        rw!("fact_one-add-3"; 
            "(VecAdd (VecMul ?x ?y) ?x)" => 
            "(VecMul ?x (VecAdd ?y 1))"
            if is_vec("?x", "?y", "?x", "?x")
        ),

        rw!("fact_one-add-4"; 
            "(VecAdd (VecMul ?y ?x) ?x)" => 
            "(VecMul (VecAdd ?y 1) ?x)"
            if is_vec("?x", "?y", "?x", "?x")
        ),

    ];
    rules
}

pub fn simplificatio_rules_min_mul() ->  Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("fact-sub-1-1-1-1"; 
            "(VecMinus (VecMul ?x ?ct_y) (VecMul ?z ?ct_y))" => 
            "(VecMul (VecMinus ?x ?z) ?ct_y)"
            if is_vec("?x", "?z", "?ct_y", "?x")
        ),

        rw!("fact-sub-1-1-1-4"; 
            "(VecMinus (VecMul ?x (VecNeg ?ct_y)) (VecMul ?z ?ct_y))" => 
            "(VecMul (VecNeg (VecAdd ?x ?z)) ?ct_y)"
            if is_vec("?x", "?z", "?ct_y", "?x")
        ),

        rw!("fact-sub-1-1-2-1"; 
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecMul ?ct_z ?pt_y))" => 
            "(VecMul (VecMinus ?ct_x ?ct_z) ?pt_y)"
            if is_vec("?ct_x", "?ct_z", "?pt_y", "?ct_x")
        ),

        rw!("fact-sub-1-2-1-1"; 
            "(VecMinus (VecMul ?x ?ct_y) (VecMul ?ct_y ?z))" => 
            "(VecMul (VecMinus ?x ?z) ?ct_y)"
            if is_vec("?x", "?ct_y", "?z", "?x")
        ),

        rw!("fact-sub-1-2-2-1"; 
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecMul ?pt_y ?ct_z))" => 
            "(VecMul (VecMinus ?ct_x ?ct_z) ?pt_y)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?ct_x")
        ),

        rw!("fact-sub-1-3-1-1"; 
            "(VecMinus (VecMul ?ct_y ?x) (VecMul ?z ?ct_y))" => 
            "(VecMul ?ct_y (VecMinus ?x ?z))"
            if is_vec("?ct_y", "?x", "?z", "?ct_y")
        ),

        rw!("fact-sub-1-3-1-4"; 
            "(VecMinus (VecMul (VecNeg ?ct_y) ?x) (VecMul ?z ?ct_y))" => 
            "(VecMul (VecNeg ?ct_y) (VecAdd ?x ?z))"
            if is_vec("?ct_y", "?x", "?z", "?ct_y")
        ),

        rw!("fact-sub-1-3-2-1"; 
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecMul ?ct_z ?pt_y))" => 
            "(VecMul ?pt_y (VecMinus ?ct_x ?ct_z))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?pt_y")
        ),

        rw!("fact-sub-1-3-2-4"; 
            "(VecMinus (VecMul (VecNeg ?pt_y) ?ct_x) (VecMul ?ct_z ?pt_y))" => 
            "(VecMul (VecNeg ?pt_y) (VecAdd ?ct_x ?ct_z))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?pt_y")
        ),

        rw!("fact-sub-1-4-1-1"; 
            "(VecMinus (VecMul ?ct_y ?x) (VecMul ?ct_y ?z))" => 
            "(VecMul ?ct_y (VecMinus ?x ?z))"
            if is_vec("?ct_y", "?x", "?z", "?ct_y")
        ),

        rw!("fact-sub-1-4-1-4"; 
            "(VecMinus (VecMul (VecNeg ?ct_y) ?x) (VecMul ?ct_y ?z))" => 
            "(VecMul (VecNeg ?ct_y) (VecAdd ?x ?z))"
            if is_vec("?ct_y", "?x", "?z", "?ct_y")
        ),

        rw!("fact-sub-1-4-2-1"; 
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecMul ?pt_y ?ct_z))" => 
            "(VecMul ?pt_y (VecMinus ?ct_x ?ct_z))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?pt_y")
        ),

        rw!("fact-sub-1-4-2-4"; 
            "(VecMinus (VecMul (VecNeg ?pt_y) ?ct_x) (VecMul ?pt_y ?ct_z))" => 
            "(VecMul (VecNeg ?pt_y) (VecAdd ?ct_x ?ct_z))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?pt_y")
        ),
        rw!("fact-sub-2-1-1"; 
            "(VecMinus (VecAdd ?u (VecMul ?x ?ct_y)) (VecMul ?z ?ct_y))" => 
            "(VecAdd ?u (VecMul (VecMinus ?x ?z) ?ct_y))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-1-2"; 
            "(VecMinus (VecAdd ?u (VecMul ?ct_x ?pt_y)) (VecMul ?ct_z ?pt_y))" => 
            "(VecAdd ?u (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y))"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-2-1"; 
            "(VecMinus (VecAdd ?u (VecMul ?x ?ct_y)) (VecMul ?ct_y ?z))" => 
            "(VecAdd ?u (VecMul (VecMinus ?x ?z) ?ct_y))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-2-2"; 
            "(VecMinus (VecAdd ?u (VecMul ?ct_x ?pt_y)) (VecMul ?pt_y ?ct_z))" => 
            "(VecAdd ?u (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y))"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-3-1"; 
            "(VecMinus (VecAdd ?u (VecMul ?ct_y ?x)) (VecMul ?z ?ct_y))" => 
            "(VecAdd ?u (VecMul ?ct_y (VecMinus ?x ?z)))"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-3-2"; 
            "(VecMinus (VecAdd ?u (VecMul ?pt_y ?ct_x)) (VecMul ?ct_z ?pt_y))" => 
            "(VecAdd ?u (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-4-1"; 
            "(VecMinus (VecAdd ?u (VecMul ?ct_y ?x)) (VecMul ?ct_y ?z))" => 
            "(VecAdd ?u (VecMul ?ct_y (VecMinus ?x ?z)))"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-4-2"; 
            "(VecMinus (VecAdd ?u (VecMul ?pt_y ?ct_x)) (VecMul ?pt_y ?ct_z))" => 
            "(VecAdd ?u (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-5-1"; 
            "(VecMinus (VecMinus ?u (VecMul ?x ?ct_y)) (VecMul ?z ?ct_y))" => 
            "(VecMinus ?u (VecMul (VecAdd ?x ?z) ?ct_y))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-5-2"; 
            "(VecMinus (VecMinus ?u (VecMul ?ct_x ?pt_y)) (VecMul ?ct_z ?pt_y))" => 
            "(VecMinus ?u (VecMul (VecAdd ?ct_x ?ct_z) ?pt_y))"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-6-1"; 
            "(VecMinus (VecMinus ?u (VecMul ?x ?ct_y)) (VecMul ?ct_y ?z))" => 
            "(VecMinus ?u (VecMul (VecAdd ?x ?z) ?ct_y))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-6-2"; 
            "(VecMinus (VecMinus ?u (VecMul ?ct_x ?pt_y)) (VecMul ?pt_y ?ct_z))" => 
            "(VecMinus ?u (VecMul (VecAdd ?ct_x ?ct_z) ?pt_y))"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-7-1"; 
            "(VecMinus (VecMinus ?u (VecMul ?ct_y ?x)) (VecMul ?z ?ct_y))" => 
            "(VecMinus ?u (VecMul ?ct_y (VecAdd ?x ?z)))"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-7-2"; 
            "(VecMinus (VecMinus ?u (VecMul ?pt_y ?ct_x)) (VecMul ?ct_z ?pt_y))" => 
            "(VecMinus ?u (VecMul ?pt_y (VecAdd ?ct_x ?ct_z)))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-8-1"; 
            "(VecMinus (VecMinus ?u (VecMul ?ct_y ?x)) (VecMul ?ct_y ?z))" => 
            "(VecMinus ?u (VecMul ?ct_y (VecAdd ?x ?z)))"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-8-2"; 
            "(VecMinus (VecMinus ?u (VecMul ?pt_y ?ct_x)) (VecMul ?pt_y ?ct_z))" => 
            "(VecMinus ?u (VecMul ?pt_y (VecAdd ?ct_x ?ct_z)))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-9-1"; 
            "(VecMinus (VecAdd (VecMul ?x ?ct_y) ?u) (VecMul ?z ?ct_y))" => 
            "(VecAdd ?u (VecMul (VecMinus ?x ?z) ?ct_y))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-9-2"; 
            "(VecMinus (VecAdd (VecMul ?ct_x ?pt_y) ?u) (VecMul ?ct_z ?pt_y))" => 
            "(VecAdd ?u (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y))"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-10-1"; 
            "(VecMinus (VecAdd (VecMul ?x ?ct_y) ?u) (VecMul ?ct_y ?z))" => 
            "(VecAdd ?u (VecMul (VecMinus ?x ?z) ?ct_y))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-10-2"; 
            "(VecMinus (VecAdd (VecMul ?ct_x ?pt_y) ?u) (VecMul ?pt_y ?ct_z))" => 
            "(VecAdd ?u (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y))"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rw!("fact-sub-2-11-1"; 
            "(VecMinus (VecAdd (VecMul ?ct_y ?x) ?u) (VecMul ?z ?ct_y))" => 
            "(VecAdd ?u (VecMul ?ct_y (VecMinus ?x ?z)))"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-11-2"; 
            "(VecMinus (VecAdd (VecMul ?pt_y ?ct_x) ?u) (VecMul ?ct_z ?pt_y))" => 
            "(VecAdd ?u (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-12-1"; 
            "(VecMinus (VecAdd (VecMul ?ct_y ?x) ?u) (VecMul ?ct_y ?z))" => 
            "(VecAdd ?u (VecMul ?ct_y (VecMinus ?x ?z)))"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-12-2"; 
            "(VecMinus (VecAdd (VecMul ?pt_y ?ct_x) ?u) (VecMul ?pt_y ?ct_z))" => 
            "(VecAdd ?u (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)))"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-13-1"; 
            "(VecMinus (VecMinus (VecMul ?x ?ct_y) ?u) (VecMul ?z ?ct_y))" => 
            "(VecMinus (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-13-2"; 
            "(VecMinus (VecMinus (VecMul ?ct_x ?pt_y) ?u) (VecMul ?ct_z ?pt_y))" => 
            "(VecMinus (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-14-1"; 
            "(VecMinus (VecMinus (VecMul ?x ?ct_y) ?u) (VecMul ?ct_y ?z))" => 
            "(VecMinus (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-14-2"; 
            "(VecMinus (VecMinus (VecMul ?ct_x ?pt_y) ?u) (VecMul ?pt_y ?ct_z))" => 
            "(VecMinus (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-15-1"; 
            "(VecMinus (VecMinus (VecMul ?ct_y ?x) ?u) (VecMul ?z ?ct_y))" => 
            "(VecMinus (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-15-2"; 
            "(VecMinus (VecMinus (VecMul ?pt_y ?ct_x) ?u) (VecMul ?ct_z ?pt_y))" => 
            "(VecMinus (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-16-1"; 
            "(VecMinus (VecMinus (VecMul ?ct_y ?x) ?u) (VecMul ?ct_y ?z))" => 
            "(VecMinus (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-16-2"; 
            "(VecMinus (VecMinus (VecMul ?pt_y ?ct_x) ?u) (VecMul ?pt_y ?ct_z))" => 
            "(VecMinus (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-17-1"; 
            "(VecMinus (VecMul ?x ?ct_y) (VecAdd ?u (VecMul ?z ?ct_y)))" => 
            "(VecMinus (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-17-2"; 
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecAdd ?u (VecMul ?ct_z ?pt_y)))" => 
            "(VecMinus (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-18-1"; 
            "(VecMinus (VecMul ?x ?ct_y) (VecAdd ?u (VecMul ?ct_y ?z)))" => 
            "(VecMinus (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),

        rw!("fact-sub-2-18-2"; 
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecAdd ?u (VecMul ?pt_y ?ct_z)))" => 
            "(VecMinus (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-19-1"; 
            "(VecMinus (VecMul ?ct_y ?x) (VecAdd ?u (VecMul ?z ?ct_y)))" => 
            "(VecMinus (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-19-2"; 
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecAdd ?u (VecMul ?ct_z ?pt_y)))" => 
            "(VecMinus (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),

        rw!("fact-sub-2-20-1"; 
            "(VecMinus (VecMul ?ct_y ?x) (VecAdd ?u (VecMul ?ct_y ?z)))" => 
            "(VecMinus (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),

        rw!("fact-sub-2-20-2"; 
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecAdd ?u (VecMul ?pt_y ?ct_z)))" => 
            "(VecMinus (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-21-1";
            "(VecMinus (VecMul ?x ?ct_y) (VecMinus ?u (VecMul ?z ?ct_y)))" =>
            "(VecMinus (VecMul (VecAdd ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),
        rewrite!("fact-sub-2-21-2";
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecMinus ?u (VecMul ?ct_z ?pt_y)))" =>
            "(VecMinus (VecMul (VecAdd ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-22-1";
            "(VecMinus (VecMul ?x ?ct_y) (VecMinus ?u (VecMul ?ct_y ?z)))" =>
            "(VecMinus (VecMul (VecAdd ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),
        rewrite!("fact-sub-2-22-2";
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecMinus ?u (VecMul ?pt_y ?ct_z)))" =>
            "(VecMinus (VecMul (VecAdd ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-23-1";
            "(VecMinus (VecMul ?ct_y ?x) (VecMinus ?u (VecMul ?z ?ct_y)))" =>
            "(VecMinus (VecMul ?ct_y (VecAdd ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),
        rewrite!("fact-sub-2-23-2";
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecMinus ?u (VecMul ?ct_z ?pt_y)))" =>
            "(VecMinus (VecMul ?pt_y (VecAdd ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-24-1";
            "(VecMinus (VecMul ?ct_y ?x) (VecMinus ?u (VecMul ?ct_y ?z)))" =>
            "(VecMinus (VecMul ?ct_y (VecAdd ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),
        rewrite!("fact-sub-2-24-2";
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecMinus ?u (VecMul ?pt_y ?ct_z)))" =>
            "(VecMinus (VecMul ?pt_y (VecAdd ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-25-1";
            "(VecMinus (VecMul ?x ?ct_y) (VecAdd (VecMul ?z ?ct_y) ?u))" =>
            "(VecMinus (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),
        rewrite!("fact-sub-2-25-2";
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecAdd (VecMul ?ct_z ?pt_y) ?u))" =>
            "(VecMinus (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-26-1";
            "(VecMinus (VecMul ?x ?ct_y) (VecAdd (VecMul ?ct_y ?z) ?u))" =>
            "(VecMinus (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),
        rewrite!("fact-sub-2-26-2";
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecAdd (VecMul ?pt_y ?ct_z) ?u))" =>
            "(VecMinus (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-27-1";
            "(VecMinus (VecMul ?ct_y ?x) (VecAdd (VecMul ?z ?ct_y) ?u))" =>
            "(VecMinus (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),
        rewrite!("fact-sub-2-27-2";
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecAdd (VecMul ?ct_z ?pt_y) ?u))" =>
            "(VecMinus (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-28-1";
            "(VecMinus (VecMul ?ct_y ?x) (VecAdd (VecMul ?ct_y ?z) ?u))" =>
            "(VecMinus (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),
        rewrite!("fact-sub-2-28-2";
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecAdd (VecMul ?pt_y ?ct_z) ?u))" =>
            "(VecMinus (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-29-1";
            "(VecMinus (VecMul ?x ?ct_y) (VecMinus (VecMul ?z ?ct_y) ?u))" =>
            "(VecAdd (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),
        rewrite!("fact-sub-2-29-2";
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecMinus (VecMul ?ct_z ?pt_y) ?u))" =>
            "(VecAdd (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-30-1";
            "(VecMinus (VecMul ?x ?ct_y) (VecMinus (VecMul ?ct_y ?z) ?u))" =>
            "(VecAdd (VecMul (VecMinus ?x ?z) ?ct_y) ?u)"
            if is_vec("?x", "?ct_y", "?z", "?u")
        ),
        rewrite!("fact-sub-2-30-2";
            "(VecMinus (VecMul ?ct_x ?pt_y) (VecMinus (VecMul ?pt_y ?ct_z) ?u))" =>
            "(VecAdd (VecMul (VecMinus ?ct_x ?ct_z) ?pt_y) ?u)"
            if is_vec("?ct_x", "?pt_y", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-31-1";
            "(VecMinus (VecMul ?ct_y ?x) (VecMinus (VecMul ?z ?ct_y) ?u))" =>
            "(VecAdd (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),
        rewrite!("fact-sub-2-31-2";
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecMinus (VecMul ?ct_z ?pt_y) ?u))" =>
            "(VecAdd (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fact-sub-2-32-1";
            "(VecMinus (VecMul ?ct_y ?x) (VecMinus (VecMul ?ct_y ?z) ?u))" =>
            "(VecAdd (VecMul ?ct_y (VecMinus ?x ?z)) ?u)"
            if is_vec("?ct_y", "?x", "?z", "?u")
        ),
        rewrite!("fact-sub-2-32-2";
            "(VecMinus (VecMul ?pt_y ?ct_x) (VecMinus (VecMul ?pt_y ?ct_z) ?u))" =>
            "(VecAdd (VecMul ?pt_y (VecMinus ?ct_x ?ct_z)) ?u)"
            if is_vec("?pt_y", "?ct_x", "?ct_z", "?u")
        ),
        rewrite!("fold-negate-sub";
            "(VecNeg (VecMinus (VecNeg ?x) ?y))" =>
            "(VecAdd ?x ?y)"
            if is_vec("?x", "?y", "?x", "?x")
        ),
        rewrite!("part-fold-dist-sub";
            "(VecMul (VecMinus ?c0 ?ct_x) ?c1)" =>
            "(VecAdd (VecMul ?ct_x (VecNeg ?c1)) (VecMul ?c0 ?c1))"
            if is_vec("?ct_x", "?c0", "?c1" , "?c1")
        ),
        rewrite!("part-fold-dist-zero_m-1";
            "(VecMul (VecNeg ?x) ?y)" =>
            "(VecMul ?x (VecNeg ?y))"
            if is_vec("?x", "?y", "?x", "?y")
        ),
    ];
    rules
}

pub fn vector_assoc_mul_rules() ->  Vec<Rewrite<VecLang, ConstantFold>> {
    let rules: Vec<Rewrite<VecLang, ConstantFold>> = vec![
        rw!("part-fold-assoc-mul"; 
        "(VecMul (VecMul ?x ?c0) ?c1)" => 
        "(VecMul ?x (VecMul ?c0 ?c1))"
        if is_vec("?x", "?c0", "?c1", "?x")
        ),
        rw!("assoc-balan-mul-1"; 
            "(VecMul (VecMul (VecMul ?x ?y) ?z) ?t)" => 
            "(VecMul (VecMul ?x ?y) (VecMul ?z ?t))"
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-mul-2"; 
            "(VecMul (VecMul ?z (VecMul ?x ?y)) ?t)" => 
            "(VecMul (VecMul ?z ?x) (VecMul ?y ?t))"
            if is_vec("?z", "?x", "?y", "?t")
        ),
        rw!("assoc-balan-mul-3"; 
            "(VecMul ?x (VecMul ?y (VecMul ?z ?t)))" => 
            "(VecMul (VecMul ?x ?y) (VecMul ?z ?t))"
            if is_vec("?x", "?y", "?z", "?t")
        ),
        rw!("assoc-balan-mul-4"; 
            "(VecMul ?x (VecMul (VecMul ?z ?t) ?y))" => 
            "(VecMul (VecMul ?x ?z) (VecMul ?t ?y))"
            if is_vec("?x", "?z", "?t", "?y")
        ),
    ];
    rules
}
