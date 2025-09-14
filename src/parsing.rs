//! Bunda-Gerth structured parsing

use crate::{Primitive, ast::Sp};

enum Token {}

enum Category {
    A,
    AMf,
    OMf,
    Df,
    N,
    AMm,
    OMm,
    Dm,
    Jot,
    Arr,
}

type Tree = Box<ParseTree>;
enum ParseTree {
    Leaf { cat: Category, t: Token },
    AlphaMonadFnCall { alpha: Tree, fun: Tree },
    OmegaMonadFnCall { omega: Tree, fun: Tree },
    Assignment { name: String, val: Tree },
    AlphaModifierFnCall { alpha: Tree, fun: Tree },
    OmegaModifierFnCall { omega: Tree, fun: Tree },
}
