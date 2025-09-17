pub(crate) use crate::parsing::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprToken<'src> {
    pub(crate) kind: ExprTokenKind<'src>,
    pub(crate) span: Sp<'src>,
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprTokenKind<'src> {
    Number,
    String,
    Char,
    PrimArray,   ArrayName,
    PrimAVerb,   PrimOVerb,   PrimDVerb,
    PrimAAdverb, PrimOAdverb, PrimDAdverb,
    AVerbName,   OVerbName,   DVerbName,
    // AAdverbName,  <-- CANNOT EXIST! There's no way to construct a primitive of this kind
    OAdverbName, DAdverbName,
    ArrayAssign, VerbAssign,  AdverbAssign,
    Parenthesized(Vec<ExprToken<'src>>),
}

impl<'src> ExprToken<'src> {
    fn to_tree(&self) -> Tree {
        let leaf = |cat| {
            Box::new(ExprTree::Leaf {
                cat,
                t: self.clone(),
            })
        };

        match &self.kind {
            ExprTokenKind::Number => leaf(Category::A),
            ExprTokenKind::String => leaf(Category::A),
            ExprTokenKind::Char => leaf(Category::A),
            ExprTokenKind::PrimArray => leaf(Category::A),
            ExprTokenKind::PrimAVerb => leaf(Category::Av),
            ExprTokenKind::PrimOVerb => leaf(Category::Ov),
            ExprTokenKind::PrimDVerb => leaf(Category::Dv),
            ExprTokenKind::PrimDAdverb => leaf(Category::Da),
            ExprTokenKind::PrimAAdverb => leaf(Category::Aa),
            ExprTokenKind::PrimOAdverb => leaf(Category::Oa),
            ExprTokenKind::ArrayName => leaf(Category::N),
            ExprTokenKind::AVerbName => leaf(Category::Av),
            ExprTokenKind::OVerbName => leaf(Category::Av),
            ExprTokenKind::OAdverbName => leaf(Category::Oa),
            ExprTokenKind::ArrayAssign => leaf(Category::Ass),
            ExprTokenKind::VerbAssign => leaf(Category::Ass),
            ExprTokenKind::AdverbAssign => leaf(Category::Ass),
            ExprTokenKind::DVerbName => leaf(Category::Dv),
            ExprTokenKind::DAdverbName => leaf(Category::Da),
            ExprTokenKind::Parenthesized(tokens) => todo!(),
        }
    }
}
type Tree<'src> = Box<ExprTree<'src>>;

/// Used to parse expressions, not top-level elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExprTree<'src> {
    Leaf {
        cat: Category,
        t: ExprToken<'src>,
    },
    AlphaVerbCall {
        verb: Tree<'src>,
        alpha: Tree<'src>,
    },
    OmegaVerbCall {
        verb: Tree<'src>,
        omega: Tree<'src>,
    },
    DyadicVerbCall {
        verb: Tree<'src>,
        alpha: Tree<'src>,
        omega: Tree<'src>,
    },
    Assignment {
        name: String,
        val: Tree<'src>,
    },
    AlphaAdverbCall {
        adverb: Tree<'src>,
        alpha: Tree<'src>,
    },
    OmegaAdverbCall {
        adverb: Tree<'src>,
        omega: Tree<'src>,
    },
    DyadicAdverbCall {
        adverb: Tree<'src>,
        alpha: Tree<'src>,
        omega: Tree<'src>,
    },
}

impl<'src> ExprTree<'src> {
    fn category(&self) -> Category {
        match self {
            ExprTree::Leaf { cat, t } => *cat,
            ExprTree::AlphaVerbCall { .. } => Category::Av,
            ExprTree::OmegaVerbCall { .. } => Category::Ov,
            ExprTree::Assignment { .. } => Category::Ass,
            ExprTree::AlphaAdverbCall { .. } => Category::Aa,
            ExprTree::OmegaAdverbCall { .. } => Category::Oa,
            ExprTree::DyadicVerbCall { .. } => Category::Dv,
            ExprTree::DyadicAdverbCall { .. } => Category::Da,
        }
    }
    fn minimum_span(&self) -> ExprToken {
        match self {
            ExprTree::Leaf { cat, t } => t.clone(),
            ExprTree::AlphaVerbCall { verb, alpha } => todo!(),
            ExprTree::OmegaVerbCall { verb, omega } => todo!(),
            ExprTree::Assignment { name, val } => todo!(),
            ExprTree::AlphaAdverbCall { adverb, alpha } => todo!(),
            ExprTree::OmegaAdverbCall { adverb, omega } => todo!(),
            ExprTree::DyadicVerbCall { verb, alpha, omega } => todo!(),
            ExprTree::DyadicAdverbCall {
                adverb,
                alpha,
                omega,
            } => todo!(),
        }
    }
}

/// `ts` must be non-empty
pub(crate) fn parse_expr<'src>(
    start_span: Sp<'src>,
    ts: &'src [ExprToken],
) -> TResult<'src, ExprTree<'src>> {
    if ts.is_empty() {
        return Err(TError {
            span: start_span,
            kind: TErrorKind::EmptyExpr,
        });
    }
    parse_expr_go(ts, 0, 0)
}
pub(crate) fn parse_expr_go<'src>(
    ts: &'src [ExprToken],
    mut cur: usize,
    min_bp: u8,
) -> TResult<'src, ExprTree<'src>> {
    use ExprTokenKind as TK;
    use TParseErrKind as EK;
    let lhs: Tree = match ts.get(cur) {
        Some(t) => t,
        None => {
            return Err(TParseErr::with_span(
                ts.last().map(|t| t.span).unwrap_or(Sp::ZERO),
                EK::UnexpectedEndOfExpression,
            ))?;
        }
    }
    .to_tree();
    cur += 1;

    loop {
        let op: Tree = match ts.get(cur) {
            Some(t) => t.to_tree(),
            None => break,
        };
        cur += 1;

        let (l_bp, l_ret) = match binding_power_of(lhs.category(), op.category()) {
            Some(bp) => bp,
            None => todo!(),
        };
        if l_bp < min_bp {
            break;
        }

        let rhs = parse_expr_go(ts, cur, min_bp)?;
        cur += 1;

        let (r_bp, r_ret) = match binding_power_of(rhs.category(), op.category()) {
            Some(bp) => bp,
            None => todo!(),
        };

        //let rhs = parse_expr_go(ts, cur + 2, r_bp);

        lhs = todo!("We have lhs, op and rhs");
    }
    Ok(*lhs)
}
