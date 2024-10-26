
use crate::*;

#[macro_export]
macro_rules! __rewrite {
    (@parse $t:ident $rhs:literal) => {
        $rhs.parse::<$crate::$t<_>>().unwrap()
    };
    (@parse $t:ident $rhs:expr) => { $rhs };
    (@applier $applier:expr;) => { $applier };
    (@applier $applier:expr; $cond:expr, $($conds:expr,)*) => {
        ConditionalApplier {
            condition: $cond,
            applier: $crate::__rewrite!(@applier $applier; $($conds,)*)
        }
    };
}

// #[macro_export]
// macro_rules! my_rewrite {
//     (
//         $name:expr;
//         $lhs:tt => $rhs:tt,
//         // $is_expensive:expr,
//         $(if $cond:expr)*
//     ) => {{
//         let searcher = $crate::__rewrite!(@parse Pattern $lhs);
//         let core_applier = $crate::__rewrite!(@parse Pattern $rhs);
//         let applier = $crate::__rewrite!(@applier core_applier; $($cond,)*);
        
//         $crate::Rewrite {
//             name: $name.to_string(),
//             searcher,
//             applier,
//             is_expensive: $is_expensive,
//         }
//     }};
    
//     (
//         $name:expr;
//         $lhs:tt <=> $rhs:tt,
//         $is_expensive:expr,
//         $(if $cond:expr)*
//     ) => {{
//         let name = $name;
//         let name2 = String::from(name.clone()) + "-rev";
//         vec![
//             $crate::my_rewrite!(name; $lhs => $rhs, $is_expensive $(if $cond)*),
//             $crate::my_rewrite!(name2; $rhs => $lhs, $is_expensive $(if $cond)*)
//         ]
//     }};
// }

#[macro_export]
macro_rules! my_rewrite {
    (
        $name:expr;
        $lhs:tt => $rhs:tt
        $(if $cond:expr)*
    )  => {{
        let searcher = __rewrite!(@parse Pattern $lhs);
        let core_applier = __rewrite!(@parse Pattern $rhs);
        let applier = __rewrite!(@applier core_applier; $($cond,)*);
        Rewrite::new($name.to_string(), searcher, applier).unwrap()
    }};
    (
        $name:expr;
        $lhs:tt <=> $rhs:tt
        $(if $cond:expr)*
    )  => {{
        let name = $name;
        let name2 = String::from(name.clone()) + "-rev";
        vec![
            $crate::my_rewrite!(name;  $lhs => $rhs $(if $cond)*),
            $crate::my_rewrite!(name2; $rhs => $lhs $(if $cond)*)
        ]
    }};
}
