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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Category {
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

const _CHECK_CATEGORY_DISCRIMINANTS: () = {
    use Category::*;
    if A as u8 != 0
        || Av as u8 != 1
        || Ov as u8 != 2
        || Dv as u8 != 3
        || N as u8 != 4
        || Aa as u8 != 5
        || Oa as u8 != 6
        || Da as u8 != 7
        || Jot as u8 != 8
        || Arr as u8 != 9
        || Ass as u8 != 10
    {
        panic!(
            "`Category` changed without updating the rest of the code!\nREMEMBER, IMPORTANT: =====> CHANGE `binding_power_of` <====="
        )
    }
};

pub(crate) fn binding_power_of<'a>(a: Category, b: Category) -> Option<(u8, ExprTree<'a>)> {
    use Category::*;
    assert!((a as u8) < 11 && (b as u8) < 11);
    const TBD: Option<(u8, ExprTree)> = None; // for now!

    #[rustfmt::skip]
    let table: [[Option<(u8, ExprTree)>; 11]; 11] = [
              /* A     αV    ⍵V    DV    N     αA   ⍵A    DA    JOT   ARR  ASS*/
        /*A*/   [None, TBD,  None, TBD,  None, TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*αV*/  [None, TBD,  None, TBD,  None, TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*⍵V*/  [TBD,  None, TBD,  None, TBD,  TBD, None, None, None, TBD, TBD],
        /*DV*/  [TBD,  None, TBD,  None, TBD,  TBD, None, None, None, TBD, TBD],
        /*N*/   [None, TBD,  None, TBD,  None, TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*αA*/  [None, TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*⍵A*/  [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*DA*/  [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*JOT*/ [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*ARR*/ [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD, TBD],
        /*ASS*/ [TBD,  TBD,  TBD,  TBD,  TBD,  TBD, TBD,  TBD,  TBD,  TBD, TBD],
    ];

    table[a as u8 as usize][b as u8 as usize].clone()
}

fn parse_expr_go<'src>(
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
