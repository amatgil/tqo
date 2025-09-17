//! Bunda-Gerth variation, heavily inspired from [TinyAPL](https://blog.rubenverg.com/tinyapl_4_parsing)
//! Structured as a tree, where each kind of branch is assigned a Category

use crate::{ast::Sp, *};
mod expr;
pub(crate) use expr::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    /// Array
    A,
    /// Alpha-monadic verb
    Av,
    /// Omega-monadic verb
    Ov,
    /// Dyadic function
    Dv,
    /// Some name/identifier
    N,
    /// Alpha-monadic adverb
    Aa,
    /// Omega-monadic adverb
    Oa,
    /// Dyadic adverb
    Da,
    /// ...should this be here?
    Jot,
    /// Arrow
    Arr,
    /// Assignment
    Ass,
}

impl Category {
    #[rustfmt::skip]
    fn binding_power_of<'a>(a: &Self, b: &Self) -> Option<(u8, ExprTree<'a>)> {
        use Category as C;
        match (a, b) {
            (C::A, C::A) => todo!(), (C::A, C::Av) => todo!(), (C::A, C::Ov) => todo!(), (C::A, C::Dv) => todo!(), (C::A, C::N) => todo!(), (C::A, C::Aa) => todo!(), (C::A, C::Oa) => todo!(), (C::A, C::Da) => todo!(), (C::A, C::Jot) => todo!(), (C::A, C::Arr) => todo!(), (C::A, C::Ass) => todo!(),
            (C::Av, C::A) => todo!(), (C::Av, C::Av) => todo!(), (C::Av, C::Ov) => todo!(), (C::Av, C::Dv) => todo!(), (C::Av, C::N) => todo!(), (C::Av, C::Aa) => todo!(), (C::Av, C::Oa) => todo!(), (C::Av, C::Da) => todo!(), (C::Av, C::Jot) => todo!(), (C::Av, C::Arr) => todo!(), (C::Av, C::Ass) => todo!(),
            (C::Ov, C::A) => todo!(), (C::Ov, C::Av) => todo!(), (C::Ov, C::Ov) => todo!(), (C::Ov, C::Dv) => todo!(), (C::Ov, C::N) => todo!(), (C::Ov, C::Aa) => todo!(), (C::Ov, C::Oa) => todo!(), (C::Ov, C::Da) => todo!(), (C::Ov, C::Jot) => todo!(), (C::Ov, C::Arr) => todo!(), (C::Ov, C::Ass) => todo!(),
            (C::Dv, C::A) => todo!(), (C::Dv, C::Av) => todo!(), (C::Dv, C::Ov) => todo!(), (C::Dv, C::Dv) => todo!(), (C::Dv, C::N) => todo!(), (C::Dv, C::Aa) => todo!(), (C::Dv, C::Oa) => todo!(), (C::Dv, C::Da) => todo!(), (C::Dv, C::Jot) => todo!(), (C::Dv, C::Arr) => todo!(), (C::Dv, C::Ass) => todo!(),
            (C::N, C::A) => todo!(), (C::N, C::Av) => todo!(), (C::N, C::Ov) => todo!(), (C::N, C::Dv) => todo!(), (C::N, C::N) => todo!(), (C::N, C::Aa) => todo!(), (C::N, C::Oa) => todo!(), (C::N, C::Da) => todo!(), (C::N, C::Jot) => todo!(), (C::N, C::Arr) => todo!(), (C::N, C::Ass) => todo!(),
            (C::Aa, C::A) => todo!(), (C::Aa, C::Av) => todo!(), (C::Aa, C::Ov) => todo!(), (C::Aa, C::Dv) => todo!(), (C::Aa, C::N) => todo!(), (C::Aa, C::Aa) => todo!(), (C::Aa, C::Oa) => todo!(), (C::Aa, C::Da) => todo!(), (C::Aa, C::Jot) => todo!(), (C::Aa, C::Arr) => todo!(), (C::Aa, C::Ass) => todo!(),
            (C::Oa, C::A) => todo!(), (C::Oa, C::Av) => todo!(), (C::Oa, C::Ov) => todo!(), (C::Oa, C::Dv) => todo!(), (C::Oa, C::N) => todo!(), (C::Oa, C::Aa) => todo!(), (C::Oa, C::Oa) => todo!(), (C::Oa, C::Da) => todo!(), (C::Oa, C::Jot) => todo!(), (C::Oa, C::Arr) => todo!(), (C::Oa, C::Ass) => todo!(),
            (C::Da, C::A) => todo!(), (C::Da, C::Av) => todo!(), (C::Da, C::Ov) => todo!(), (C::Da, C::Dv) => todo!(), (C::Da, C::N) => todo!(), (C::Da, C::Aa) => todo!(), (C::Da, C::Oa) => todo!(), (C::Da, C::Da) => todo!(), (C::Da, C::Jot) => todo!(), (C::Da, C::Arr) => todo!(), (C::Da, C::Ass) => todo!(),
            (C::Jot, C::A) => todo!(), (C::Jot, C::Av) => todo!(), (C::Jot, C::Ov) => todo!(), (C::Jot, C::Dv) => todo!(), (C::Jot, C::N) => todo!(), (C::Jot, C::Aa) => todo!(), (C::Jot, C::Oa) => todo!(), (C::Jot, C::Da) => todo!(), (C::Jot, C::Jot) => todo!(), (C::Jot, C::Arr) => todo!(), (C::Jot, C::Ass) => todo!(),
            (C::Arr, C::A) => todo!(), (C::Arr, C::Av) => todo!(), (C::Arr, C::Ov) => todo!(), (C::Arr, C::Dv) => todo!(), (C::Arr, C::N) => todo!(), (C::Arr, C::Aa) => todo!(), (C::Arr, C::Oa) => todo!(), (C::Arr, C::Da) => todo!(), (C::Arr, C::Jot) => todo!(), (C::Arr, C::Arr) => todo!(), (C::Arr, C::Ass) => todo!(),
            (C::Ass, C::A) => todo!(), (C::Ass, C::Av) => todo!(), (C::Ass, C::Ov) => todo!(), (C::Ass, C::Dv) => todo!(), (C::Ass, C::N) => todo!(), (C::Ass, C::Aa) => todo!(), (C::Ass, C::Oa) => todo!(), (C::Ass, C::Da) => todo!(), (C::Ass, C::Jot) => todo!(), (C::Ass, C::Arr) => todo!(), (C::Ass, C::Ass) => todo!(),
        }
    }
    #[rustfmt::skip]
    fn binding_power_of_inside_train<'a>(a: &Self, b: &Self) -> Option<(u8, ExprTree<'a>)> {
        use Category as C;
        match (a, b) {
            (C::A, C::A) => todo!(), (C::A, C::Av) => todo!(), (C::A, C::Ov) => todo!(), (C::A, C::Dv) => todo!(), (C::A, C::N) => todo!(), (C::A, C::Aa) => todo!(), (C::A, C::Oa) => todo!(), (C::A, C::Da) => todo!(), (C::A, C::Jot) => todo!(), (C::A, C::Arr) => todo!(), (C::A, C::Ass) => todo!(),
            (C::Av, C::A) => todo!(), (C::Av, C::Av) => todo!(), (C::Av, C::Ov) => todo!(), (C::Av, C::Dv) => todo!(), (C::Av, C::N) => todo!(), (C::Av, C::Aa) => todo!(), (C::Av, C::Oa) => todo!(), (C::Av, C::Da) => todo!(), (C::Av, C::Jot) => todo!(), (C::Av, C::Arr) => todo!(), (C::Av, C::Ass) => todo!(),
            (C::Ov, C::A) => todo!(), (C::Ov, C::Av) => todo!(), (C::Ov, C::Ov) => todo!(), (C::Ov, C::Dv) => todo!(), (C::Ov, C::N) => todo!(), (C::Ov, C::Aa) => todo!(), (C::Ov, C::Oa) => todo!(), (C::Ov, C::Da) => todo!(), (C::Ov, C::Jot) => todo!(), (C::Ov, C::Arr) => todo!(), (C::Ov, C::Ass) => todo!(),
            (C::Dv, C::A) => todo!(), (C::Dv, C::Av) => todo!(), (C::Dv, C::Ov) => todo!(), (C::Dv, C::Dv) => todo!(), (C::Dv, C::N) => todo!(), (C::Dv, C::Aa) => todo!(), (C::Dv, C::Oa) => todo!(), (C::Dv, C::Da) => todo!(), (C::Dv, C::Jot) => todo!(), (C::Dv, C::Arr) => todo!(), (C::Dv, C::Ass) => todo!(),
            (C::N, C::A) => todo!(), (C::N, C::Av) => todo!(), (C::N, C::Ov) => todo!(), (C::N, C::Dv) => todo!(), (C::N, C::N) => todo!(), (C::N, C::Aa) => todo!(), (C::N, C::Oa) => todo!(), (C::N, C::Da) => todo!(), (C::N, C::Jot) => todo!(), (C::N, C::Arr) => todo!(), (C::N, C::Ass) => todo!(),
            (C::Aa, C::A) => todo!(), (C::Aa, C::Av) => todo!(), (C::Aa, C::Ov) => todo!(), (C::Aa, C::Dv) => todo!(), (C::Aa, C::N) => todo!(), (C::Aa, C::Aa) => todo!(), (C::Aa, C::Oa) => todo!(), (C::Aa, C::Da) => todo!(), (C::Aa, C::Jot) => todo!(), (C::Aa, C::Arr) => todo!(), (C::Aa, C::Ass) => todo!(),
            (C::Oa, C::A) => todo!(), (C::Oa, C::Av) => todo!(), (C::Oa, C::Ov) => todo!(), (C::Oa, C::Dv) => todo!(), (C::Oa, C::N) => todo!(), (C::Oa, C::Aa) => todo!(), (C::Oa, C::Oa) => todo!(), (C::Oa, C::Da) => todo!(), (C::Oa, C::Jot) => todo!(), (C::Oa, C::Arr) => todo!(), (C::Oa, C::Ass) => todo!(),
            (C::Da, C::A) => todo!(), (C::Da, C::Av) => todo!(), (C::Da, C::Ov) => todo!(), (C::Da, C::Dv) => todo!(), (C::Da, C::N) => todo!(), (C::Da, C::Aa) => todo!(), (C::Da, C::Oa) => todo!(), (C::Da, C::Da) => todo!(), (C::Da, C::Jot) => todo!(), (C::Da, C::Arr) => todo!(), (C::Da, C::Ass) => todo!(),
            (C::Jot, C::A) => todo!(), (C::Jot, C::Av) => todo!(), (C::Jot, C::Ov) => todo!(), (C::Jot, C::Dv) => todo!(), (C::Jot, C::N) => todo!(), (C::Jot, C::Aa) => todo!(), (C::Jot, C::Oa) => todo!(), (C::Jot, C::Da) => todo!(), (C::Jot, C::Jot) => todo!(), (C::Jot, C::Arr) => todo!(), (C::Jot, C::Ass) => todo!(),
            (C::Arr, C::A) => todo!(), (C::Arr, C::Av) => todo!(), (C::Arr, C::Ov) => todo!(), (C::Arr, C::Dv) => todo!(), (C::Arr, C::N) => todo!(), (C::Arr, C::Aa) => todo!(), (C::Arr, C::Oa) => todo!(), (C::Arr, C::Da) => todo!(), (C::Arr, C::Jot) => todo!(), (C::Arr, C::Arr) => todo!(), (C::Arr, C::Ass) => todo!(),
            (C::Ass, C::A) => todo!(), (C::Ass, C::Av) => todo!(), (C::Ass, C::Ov) => todo!(), (C::Ass, C::Dv) => todo!(), (C::Ass, C::N) => todo!(), (C::Ass, C::Aa) => todo!(), (C::Ass, C::Oa) => todo!(), (C::Ass, C::Da) => todo!(), (C::Ass, C::Jot) => todo!(), (C::Ass, C::Arr) => todo!(), (C::Ass, C::Ass) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TParseErr<'src> {
    span: Sp<'src>,
    kind: TParseErrKind,
}

impl<'src> TParseErr<'src> {
    fn at(t: ExprToken<'src>, kind: TParseErrKind) -> Self {
        Self { kind, span: t.span }
    }
    fn with_span(span: Sp<'src>, kind: TParseErrKind) -> Self {
        Self { kind, span }
    }
}
#[derive(Debug, Clone)]
pub enum TParseErrKind {
    UnexpectedEndOfExpression,
}

#[test]
fn babys_first_parsing() {
    use expr::*;
    let _source = "1+2"; // We do not test the lexer here
    let ts = vec![
        ExprToken {
            kind: ExprTokenKind::Number,
            span: Sp::new(0, 1),
        },
        ExprToken {
            kind: ExprTokenKind::PrimDVerb,
            span: Sp::new(1, 2),
        },
        ExprToken {
            kind: ExprTokenKind::Number,
            span: Sp::new(2, 3),
        },
    ];

    let expr = parse_expr(Sp::ZERO, &ts).unwrap();
    let expected = ExprTree::DyadicVerbCall {
        verb: Box::new(ExprTree::Leaf {
            cat: Category::Dv,
            t: ExprToken {
                kind: ExprTokenKind::PrimDVerb,
                span: Sp::new(0, 1),
            },
        }),
        alpha: Box::new(ExprTree::Leaf {
            cat: Category::A,
            t: ExprToken {
                kind: ExprTokenKind::Number,
                span: Sp::new(0, 1),
            },
        }),
        omega: Box::new(ExprTree::Leaf {
            cat: Category::A,
            t: ExprToken {
                kind: ExprTokenKind::Number,
                span: Sp::new(0, 1),
            },
        }),
    };
    assert_eq!(expr, expected);
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
        ((Av, Da), (Da, Dv)),
        ((Dv, Da), (Da, Dv)),
        ((Dv, Jot), (Da, Dv)),
    ];

    for (lesser, greater) in gt_conditions {
        let lesser_prio = table[lesser.0 as u8 as usize][lesser.1 as u8 as usize];
        let greater_prio = table[greater.0 as u8 as usize][greater.1 as u8 as usize];

        assert!(lesser_prio.is_none() || greater_prio.unwrap() > lesser_prio.unwrap())
    }
}
