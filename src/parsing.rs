//! Bunda-Gerth variation, heavily inspired from [TinyAPL](https://blog.rubenverg.com/tinyapl_4_parsing)
//! Structured as a tree, where each kind of branch is assigned a Category

use crate::{ast::Sp, *};

#[derive(Clone)]
pub enum Token {
    Number,
    String,
    Char,
    PrimArray,
    PrimAVerb,
    PrimOVerb,
    PrimAAdverb,
    PrimOAdverb,
    ArrayName,
    AVerbName,
    OVerbName,
    // AAdverbName,  <-- CANNOT EXIST! There's no way to construct a primitive of this kind
    OAdverbName,
    ArrayAssign,
    VerbAssign,
    AdverbAssign,
    Parenthesized(Vec<Sp<Token>>),
}

impl Token {
    fn to_tree(t: Sp<Token>) -> Tree {
        let leaf = |cat| Box::new(ExprParseTree::Leaf { cat, t: t.clone() });

        match t.value {
            Token::Number => leaf(Category::A),
            Token::String => leaf(Category::A),
            Token::Char => leaf(Category::A),
            Token::PrimArray => leaf(Category::A),
            Token::PrimAVerb => leaf(Category::AMf),
            Token::PrimOVerb => leaf(Category::OMf),
            Token::PrimAAdverb => leaf(Category::AMm),
            Token::PrimOAdverb => leaf(Category::OMm),
            Token::ArrayName => leaf(Category::N),
            Token::AVerbName => leaf(Category::AMf),
            Token::OVerbName => leaf(Category::AMf),
            Token::OAdverbName => leaf(Category::OMm),
            Token::ArrayAssign => leaf(Category::Ass),
            Token::VerbAssign => leaf(Category::Ass),
            Token::AdverbAssign => leaf(Category::Ass),
            Token::Parenthesized(tokens) => todo!(),
        }
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
        use Category as C;
        match (a, b) {
            (C::A, C::A) => todo!(), (C::A, C::AMf) => todo!(), (C::A, C::OMf) => todo!(), (C::A, C::Df) => todo!(), (C::A, C::N) => todo!(), (C::A, C::AMm) => todo!(), (C::A, C::OMm) => todo!(), (C::A, C::Dm) => todo!(), (C::A, C::Jot) => todo!(), (C::A, C::Arr) => todo!(), (C::A, C::Ass) => todo!(),
            (C::AMf, C::A) => todo!(), (C::AMf, C::AMf) => todo!(), (C::AMf, C::OMf) => todo!(), (C::AMf, C::Df) => todo!(), (C::AMf, C::N) => todo!(), (C::AMf, C::AMm) => todo!(), (C::AMf, C::OMm) => todo!(), (C::AMf, C::Dm) => todo!(), (C::AMf, C::Jot) => todo!(), (C::AMf, C::Arr) => todo!(), (C::AMf, C::Ass) => todo!(),
            (C::OMf, C::A) => todo!(), (C::OMf, C::AMf) => todo!(), (C::OMf, C::OMf) => todo!(), (C::OMf, C::Df) => todo!(), (C::OMf, C::N) => todo!(), (C::OMf, C::AMm) => todo!(), (C::OMf, C::OMm) => todo!(), (C::OMf, C::Dm) => todo!(), (C::OMf, C::Jot) => todo!(), (C::OMf, C::Arr) => todo!(), (C::OMf, C::Ass) => todo!(),
            (C::Df, C::A) => todo!(), (C::Df, C::AMf) => todo!(), (C::Df, C::OMf) => todo!(), (C::Df, C::Df) => todo!(), (C::Df, C::N) => todo!(), (C::Df, C::AMm) => todo!(), (C::Df, C::OMm) => todo!(), (C::Df, C::Dm) => todo!(), (C::Df, C::Jot) => todo!(), (C::Df, C::Arr) => todo!(), (C::Df, C::Ass) => todo!(),
            (C::N, C::A) => todo!(), (C::N, C::AMf) => todo!(), (C::N, C::OMf) => todo!(), (C::N, C::Df) => todo!(), (C::N, C::N) => todo!(), (C::N, C::AMm) => todo!(), (C::N, C::OMm) => todo!(), (C::N, C::Dm) => todo!(), (C::N, C::Jot) => todo!(), (C::N, C::Arr) => todo!(), (C::N, C::Ass) => todo!(),
            (C::AMm, C::A) => todo!(), (C::AMm, C::AMf) => todo!(), (C::AMm, C::OMf) => todo!(), (C::AMm, C::Df) => todo!(), (C::AMm, C::N) => todo!(), (C::AMm, C::AMm) => todo!(), (C::AMm, C::OMm) => todo!(), (C::AMm, C::Dm) => todo!(), (C::AMm, C::Jot) => todo!(), (C::AMm, C::Arr) => todo!(), (C::AMm, C::Ass) => todo!(),
            (C::OMm, C::A) => todo!(), (C::OMm, C::AMf) => todo!(), (C::OMm, C::OMf) => todo!(), (C::OMm, C::Df) => todo!(), (C::OMm, C::N) => todo!(), (C::OMm, C::AMm) => todo!(), (C::OMm, C::OMm) => todo!(), (C::OMm, C::Dm) => todo!(), (C::OMm, C::Jot) => todo!(), (C::OMm, C::Arr) => todo!(), (C::OMm, C::Ass) => todo!(),
            (C::Dm, C::A) => todo!(), (C::Dm, C::AMf) => todo!(), (C::Dm, C::OMf) => todo!(), (C::Dm, C::Df) => todo!(), (C::Dm, C::N) => todo!(), (C::Dm, C::AMm) => todo!(), (C::Dm, C::OMm) => todo!(), (C::Dm, C::Dm) => todo!(), (C::Dm, C::Jot) => todo!(), (C::Dm, C::Arr) => todo!(), (C::Dm, C::Ass) => todo!(),
            (C::Jot, C::A) => todo!(), (C::Jot, C::AMf) => todo!(), (C::Jot, C::OMf) => todo!(), (C::Jot, C::Df) => todo!(), (C::Jot, C::N) => todo!(), (C::Jot, C::AMm) => todo!(), (C::Jot, C::OMm) => todo!(), (C::Jot, C::Dm) => todo!(), (C::Jot, C::Jot) => todo!(), (C::Jot, C::Arr) => todo!(), (C::Jot, C::Ass) => todo!(),
            (C::Arr, C::A) => todo!(), (C::Arr, C::AMf) => todo!(), (C::Arr, C::OMf) => todo!(), (C::Arr, C::Df) => todo!(), (C::Arr, C::N) => todo!(), (C::Arr, C::AMm) => todo!(), (C::Arr, C::OMm) => todo!(), (C::Arr, C::Dm) => todo!(), (C::Arr, C::Jot) => todo!(), (C::Arr, C::Arr) => todo!(), (C::Arr, C::Ass) => todo!(),
            (C::Ass, C::A) => todo!(), (C::Ass, C::AMf) => todo!(), (C::Ass, C::OMf) => todo!(), (C::Ass, C::Df) => todo!(), (C::Ass, C::N) => todo!(), (C::Ass, C::AMm) => todo!(), (C::Ass, C::OMm) => todo!(), (C::Ass, C::Dm) => todo!(), (C::Ass, C::Jot) => todo!(), (C::Ass, C::Arr) => todo!(), (C::Ass, C::Ass) => todo!(),
        }
    }
}

type Tree = Box<ExprParseTree>;
/// Used to parse expressions, not top-level elements
/// TODO: Add Sp<Token> to each branch to be able to track the origin?
/// Or, i think span should just be Sp, not Sp<T> <--- definitely this
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
            ExprParseTree::Leaf { cat, t } => t.clone(),
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

#[test]
fn bunda_gerth_binding_powers() {
    // See README.md/Bunda-Gerth for the actual table
    let tbd = None; // for now!
    #[rustfmt::skip]
    let table: [[Option<u8>; 10]; 10] = [
              /* A     αMF   ⍵MF   DF    N     αMM  ⍵MM   DM    JOT   ARR */
        /*A*/   [None, tbd,  None, tbd,  None, tbd, tbd,  tbd,  tbd,  tbd],
        /*αMF*/ [None, tbd,  None, tbd,  None, tbd, tbd,  tbd,  tbd,  tbd],
        /*⍵MF*/ [tbd,  None, tbd,  None, tbd,  tbd, None, None, None, tbd],
        /*DF*/  [tbd,  None, tbd,  None, tbd,  tbd, None, None, None, tbd],
        /*N*/   [None, tbd,  None, tbd,  None, tbd, tbd,  tbd,  tbd,  tbd],
        /*αMM*/ [None, tbd,  tbd,  tbd,  tbd,  tbd, tbd,  tbd,  tbd,  tbd],
        /*⍵MM*/ [tbd,  tbd,  tbd,  tbd,  tbd,  tbd, tbd,  tbd,  tbd,  tbd],
        /*DM*/  [tbd,  tbd,  tbd,  tbd,  tbd,  tbd, tbd,  tbd,  tbd,  tbd],
        /*JOT*/ [tbd,  tbd,  tbd,  tbd,  tbd,  tbd, tbd,  tbd,  tbd,  tbd],
        /*ARR*/ [tbd,  tbd,  tbd,  tbd,  tbd,  tbd, tbd,  tbd,  tbd,  tbd],
    ];

    use Category::*;

    let gt_conditions = [
        ((AMf, Dm), (Dm, Df)),
        ((Df, Dm), (Dm, Df)),
        ((Df, Jot), (Dm, Df)),
    ];

    for (lesser, greater) in gt_conditions {
        let lesser_prio = table[lesser.0 as u8 as usize][lesser.1 as u8 as usize];
        let greater_prio = table[greater.0 as u8 as usize][greater.1 as u8 as usize];

        assert!(lesser_prio.is_none() || greater_prio.unwrap() > lesser_prio.unwrap())
    }
}
