//! Bunda-Gerth variation, inspired from [TinyAPL](https://blog.rubenverg.com/tinyapl_4_parsing)
//! and [the paper itself](https://dl.acm.org/doi/pdf/10.1145/384283.801081)
//! Structured as a tree, where each kind of branch is assigned a Category

use crate::{ast::Sp, *};
use std::mem;
mod expr;
pub(crate) use expr::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
    /// Arrow (in-line assignment exists)
    Arr,
    /// Assignment
    Ass,
}

fn binding_power_of<'a>(a: Category, b: Category) -> Option<(u8, ExprTree<'a>)> {
    assert!(
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            == [
                A as u8, Av as u8, Ov as u8, Dv as u8, N as u8, Aa as u8, Oa as u8, Da as u8,
                Jot as u8, Arr as u8, Ass as u8
            ]
    );
    use Category::*;
    const TBD: Option<(u8, ExprTree)> = None; // for now!

    #[rustfmt::skip]
    let table: [[Option<(u8, ExprTree)>; 10]; 10] = [
              /* A     αV    ⍵V    DV    N     αA   ⍵A    DA    JOT   ARR */
        /*A*/   [None, TBD,  None, TBD,  None, TBD, TBD,  TBD,  TBD,  TBD],
        /*αV*/  [None, TBD,  None, TBD,  None, TBD, TBD,  TBD,  TBD,  TBD],
        /*⍵V*/  [TBD,  None, TBD,  None, TBD,  TBD, None, None, None, TBD],
        /*DV*/  [TBD,  None, TBD,  None, TBD,  TBD, None, None, None, TBD],
        /*N*/   [None, TBD,  None, TBD,  None, TBD, TBD,  TBD,  TBD,  TBD],
        /*αA*/  [None, TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD],
        /*⍵A*/  [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD],
        /*DA*/  [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD],
        /*JOT*/ [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD],
        /*ARR*/ [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD],
    ];
    todo!()
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

    use Category::*;

    let gt_conditions = [
        ((Av, Da), (Da, Dv)),
        ((Dv, Da), (Da, Dv)),
        ((Dv, Jot), (Da, Dv)),
    ];

    for (lesser, greater) in gt_conditions {
        let lesser_prio = binding_power_of(lesser.0, lesser.1);
        let greater_prio = binding_power_of(greater.0, greater.1);

        assert!(lesser_prio.is_none() || greater_prio.unwrap().0 > lesser_prio.unwrap().0)
    }
}
