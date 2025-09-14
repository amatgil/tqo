//! Bunda-Gerth variation, heavily inspired from [TinyAPL](https://blog.rubenverg.com/tinyapl_4_parsing)
//! Structured as a tree, where each kind of branch is assigned a Category

use crate::{ast::Sp, *};

#[derive(Clone, Copy)]
enum Token {}

impl Token {
    fn to_tree(&self) -> Tree {
        match *self {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Category {
    /// Array
    A,
    /// Alpha-monadic function
    AMf,
    /// Omega-monadic function
    OMf,
    /// Dyadic function
    Df,
    /// Some name/identifier
    N,
    /// Alpha-monadic modifier
    AMm,
    /// Omega-monadic modifier
    OMm,
    /// Dyadic modifier
    Dm,
    /// ...should this be here?
    Jot,
    /// Arrow
    Arr,
    /// Assignment
    Ass,
}

impl Category {
    #[rustfmt::skip]
    fn binding_power_of(a: &Self, b: &Self) -> Option<(u8, ExprParseTree)> {
        match (a, b) {
            (Category::A, Category::A) => todo!(), (Category::A, Category::AMf) => todo!(), (Category::A, Category::OMf) => todo!(), (Category::A, Category::Df) => todo!(), (Category::A, Category::N) => todo!(), (Category::A, Category::AMm) => todo!(), (Category::A, Category::OMm) => todo!(), (Category::A, Category::Dm) => todo!(), (Category::A, Category::Jot) => todo!(), (Category::A, Category::Arr) => todo!(), (Category::A, Category::Ass) => todo!(),
            (Category::AMf, Category::A) => todo!(), (Category::AMf, Category::AMf) => todo!(), (Category::AMf, Category::OMf) => todo!(), (Category::AMf, Category::Df) => todo!(), (Category::AMf, Category::N) => todo!(), (Category::AMf, Category::AMm) => todo!(), (Category::AMf, Category::OMm) => todo!(), (Category::AMf, Category::Dm) => todo!(), (Category::AMf, Category::Jot) => todo!(), (Category::AMf, Category::Arr) => todo!(), (Category::AMf, Category::Ass) => todo!(),
            (Category::OMf, Category::A) => todo!(), (Category::OMf, Category::AMf) => todo!(), (Category::OMf, Category::OMf) => todo!(), (Category::OMf, Category::Df) => todo!(), (Category::OMf, Category::N) => todo!(), (Category::OMf, Category::AMm) => todo!(), (Category::OMf, Category::OMm) => todo!(), (Category::OMf, Category::Dm) => todo!(), (Category::OMf, Category::Jot) => todo!(), (Category::OMf, Category::Arr) => todo!(), (Category::OMf, Category::Ass) => todo!(),
            (Category::Df, Category::A) => todo!(), (Category::Df, Category::AMf) => todo!(), (Category::Df, Category::OMf) => todo!(), (Category::Df, Category::Df) => todo!(), (Category::Df, Category::N) => todo!(), (Category::Df, Category::AMm) => todo!(), (Category::Df, Category::OMm) => todo!(), (Category::Df, Category::Dm) => todo!(), (Category::Df, Category::Jot) => todo!(), (Category::Df, Category::Arr) => todo!(), (Category::Df, Category::Ass) => todo!(),
            (Category::N, Category::A) => todo!(), (Category::N, Category::AMf) => todo!(), (Category::N, Category::OMf) => todo!(), (Category::N, Category::Df) => todo!(), (Category::N, Category::N) => todo!(), (Category::N, Category::AMm) => todo!(), (Category::N, Category::OMm) => todo!(), (Category::N, Category::Dm) => todo!(), (Category::N, Category::Jot) => todo!(), (Category::N, Category::Arr) => todo!(), (Category::N, Category::Ass) => todo!(),
            (Category::AMm, Category::A) => todo!(), (Category::AMm, Category::AMf) => todo!(), (Category::AMm, Category::OMf) => todo!(), (Category::AMm, Category::Df) => todo!(), (Category::AMm, Category::N) => todo!(), (Category::AMm, Category::AMm) => todo!(), (Category::AMm, Category::OMm) => todo!(), (Category::AMm, Category::Dm) => todo!(), (Category::AMm, Category::Jot) => todo!(), (Category::AMm, Category::Arr) => todo!(), (Category::AMm, Category::Ass) => todo!(),
            (Category::OMm, Category::A) => todo!(), (Category::OMm, Category::AMf) => todo!(), (Category::OMm, Category::OMf) => todo!(), (Category::OMm, Category::Df) => todo!(), (Category::OMm, Category::N) => todo!(), (Category::OMm, Category::AMm) => todo!(), (Category::OMm, Category::OMm) => todo!(), (Category::OMm, Category::Dm) => todo!(), (Category::OMm, Category::Jot) => todo!(), (Category::OMm, Category::Arr) => todo!(), (Category::OMm, Category::Ass) => todo!(),
            (Category::Dm, Category::A) => todo!(), (Category::Dm, Category::AMf) => todo!(), (Category::Dm, Category::OMf) => todo!(), (Category::Dm, Category::Df) => todo!(), (Category::Dm, Category::N) => todo!(), (Category::Dm, Category::AMm) => todo!(), (Category::Dm, Category::OMm) => todo!(), (Category::Dm, Category::Dm) => todo!(), (Category::Dm, Category::Jot) => todo!(), (Category::Dm, Category::Arr) => todo!(), (Category::Dm, Category::Ass) => todo!(),
            (Category::Jot, Category::A) => todo!(), (Category::Jot, Category::AMf) => todo!(), (Category::Jot, Category::OMf) => todo!(), (Category::Jot, Category::Df) => todo!(), (Category::Jot, Category::N) => todo!(), (Category::Jot, Category::AMm) => todo!(), (Category::Jot, Category::OMm) => todo!(), (Category::Jot, Category::Dm) => todo!(), (Category::Jot, Category::Jot) => todo!(), (Category::Jot, Category::Arr) => todo!(), (Category::Jot, Category::Ass) => todo!(),
            (Category::Arr, Category::A) => todo!(), (Category::Arr, Category::AMf) => todo!(), (Category::Arr, Category::OMf) => todo!(), (Category::Arr, Category::Df) => todo!(), (Category::Arr, Category::N) => todo!(), (Category::Arr, Category::AMm) => todo!(), (Category::Arr, Category::OMm) => todo!(), (Category::Arr, Category::Dm) => todo!(), (Category::Arr, Category::Jot) => todo!(), (Category::Arr, Category::Arr) => todo!(), (Category::Arr, Category::Ass) => todo!(),
            (Category::Ass, Category::A) => todo!(), (Category::Ass, Category::AMf) => todo!(), (Category::Ass, Category::OMf) => todo!(), (Category::Ass, Category::Df) => todo!(), (Category::Ass, Category::N) => todo!(), (Category::Ass, Category::AMm) => todo!(), (Category::Ass, Category::OMm) => todo!(), (Category::Ass, Category::Dm) => todo!(), (Category::Ass, Category::Jot) => todo!(), (Category::Ass, Category::Arr) => todo!(), (Category::Ass, Category::Ass) => todo!(),
        }
    }
}

type Tree = Box<ExprParseTree>;
/// Used to parse expressions, not top-level elements
enum ExprParseTree {
    Leaf { cat: Category, t: Sp<Token> },
    AlphaMonadFnCall { alpha: Tree, fun: Tree },
    OmegaMonadFnCall { omega: Tree, fun: Tree },
    Assignment { name: String, val: Tree },
    AlphaModifierCall { alpha: Tree, fun: Tree },
    OmegaModifierCall { omega: Tree, fun: Tree },
}

impl ExprParseTree {
    fn category(&self) -> Category {
        match self {
            ExprParseTree::Leaf { cat, t } => *cat,
            ExprParseTree::AlphaMonadFnCall { .. } => Category::AMf,
            ExprParseTree::OmegaMonadFnCall { .. } => Category::OMf,
            ExprParseTree::Assignment { .. } => Category::Ass,
            ExprParseTree::AlphaModifierCall { .. } => Category::AMm,
            ExprParseTree::OmegaModifierCall { .. } => Category::OMm,
        }
    }
    fn minimum_span(&self) -> Sp<Token> {
        match self {
            ExprParseTree::Leaf { cat, t } => *t,
            ExprParseTree::AlphaMonadFnCall { alpha, fun } => todo!(),
            ExprParseTree::OmegaMonadFnCall { omega, fun } => todo!(),
            ExprParseTree::Assignment { name, val } => todo!(),
            ExprParseTree::AlphaModifierCall { alpha, fun } => todo!(),
            ExprParseTree::OmegaModifierCall { omega, fun } => todo!(),
        }
    }
    fn pairs(trees: &[Self]) -> Vec<(u8, ExprParseTree)> {
        let mut pairs = vec![];
        for pair in trees.windows(2) {
            let [left, right] = pair else { unreachable!() };
            if let Some(p) = Category::binding_power_of(&left.category(), &right.category()) {
                pairs.push(p);
            }
        }
        pairs
    }
}
