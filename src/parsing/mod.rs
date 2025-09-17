//! Bunda-Gerth variation, inspired from [TinyAPL](https://blog.rubenverg.com/tinyapl_4_parsing)
//! and [the paper itself](https://dl.acm.org/doi/pdf/10.1145/384283.801081)
//! Structured as a tree, where each kind of branch is assigned a Category

use crate::{ast::Sp, *};
use std::mem;
mod expr;
pub(crate) use expr::*;

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

    let expr = parse_expr(&ts, Sp::ZERO ).unwrap();
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

//#[test]
//fn bunda_gerth_binding_powers() {
//    // See README.md/Bunda-Gerth for the actual table
//
//    use Category::*;
//
//    let gt_conditions = [
//        ((Av, Da), (Da, Dv)),
//        ((Dv, Da), (Da, Dv)),
//        ((Dv, Jot), (Da, Dv)),
//    ];
//
//    for (lesser, greater) in gt_conditions {
//        let lesser_prio = binding_power_of(lesser.0, lesser.1);
//        let greater_prio = binding_power_of(greater.0, greater.1);
//
//        assert!(lesser_prio.is_none() || greater_prio.unwrap().0 > lesser_prio.unwrap().0)
//    }
//}
