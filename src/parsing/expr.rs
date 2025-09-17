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
pub(crate) fn binding_power_of<'src>(
    a: &ExprTree<'src>,
    b: &ExprTree<'src>,
) -> Option<(u8, ExprTree<'src>)> {
    let (a, b) = (a.clone(), b.clone());
    let (ac, bc) = (a.category(), b.category());
    assert!((ac as u8) < 11 && (bc as u8) < 11);
    let s = |bp, t| Some((bp, t));
    let tbd = None; // for now!

    fn merge_with_cat<'src>(
        cat: Category,
        left: ExprTree<'src>,
        right: ExprTree<'src>,
    ) -> ExprTree<'src> {
        match cat {
            Category::A | Category::N => unreachable!(),
            Category::Av => ExprTree::AlphaVerbCall {
                alpha: Box::new(left),
                verb: Box::new(right),
            },
            Category::Ov => ExprTree::OmegaVerbCall {
                verb: Box::new(left),
                omega: Box::new(right),
            },
            Category::Dv => unreachable!("i am honestly not really sure"),
            Category::Aa => ExprTree::AlphaAdverbCall {
                alpha: Box::new(left),
                adverb: Box::new(right),
            },
            Category::Oa => ExprTree::OmegaAdverbCall {
                adverb: Box::new(left),
                omega: Box::new(right),
            },
            Category::Da => todo!(),
            Category::Jot => unreachable!(),
            Category::Arr => unreachable!(),
            Category::Ass => todo!(),
        }
    }

    #[rustfmt::skip]
    let table: [[Option<_>; 11]; 11] = [
              /* A         αV    ⍵V    DV         N     αA   ⍵A    DA    JOT   ARR  ASS*/
        /*A*/   [None,         tbd.clone(),  None,         s(2, ExprTree::OmegaVerbCall { verb: Box::new(a), omega: Box::new(b) }),  None, tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*αV*/  [None,         tbd.clone(),  None,         tbd.clone(),       None, tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*⍵V*/  [tbd.clone(),  None,         tbd.clone(),  None,              tbd.clone(),  tbd.clone(), None, None, None, tbd.clone(), tbd.clone()],
        /*DV*/  [tbd.clone(),  None,         tbd.clone(),  None,              tbd.clone(),  tbd.clone(), None, None, None, tbd.clone(), tbd.clone()],
        /*N*/   [None,         tbd.clone(),  None,         tbd.clone(),       None, tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*αA*/  [None,         tbd.clone(),  tbd.clone(),  tbd.clone(),       tbd.clone(),  tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*⍵A*/  [tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(),       tbd.clone(),  tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*DA*/  [tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(),       tbd.clone(),  tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*JOT*/ [tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(),       tbd.clone(),  tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*ARR*/ [tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(),       tbd.clone(),  tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
        /*ASS*/ [tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(),       tbd.clone(),  tbd.clone(), tbd.clone(),  tbd.clone(),  tbd.clone(),  tbd.clone(), tbd.clone()],
    ];

    table[ac as u8 as usize][bc as u8 as usize].clone()
}

fn parse_expr_go<'src>(
    mut ts: Vec<ExprTree<'src>>,
    start: Sp<'src>,
) -> TResult<'src, ExprTree<'src>> {
    use TParseErrKind as EK;

    // NOTE: `cur` always points to the (would-be) op
    let mut marker = 0;
    loop {
        dbg!(marker);
        if ts.len() == 1 {
            return Ok(ts[0].clone());
        }
        let current = match ts.get(marker) {
            Some(t) => t,
            None => {
                return Err(TParseErr::with_span(
                    ts.last().cloned().map(|t| t.span()).unwrap_or(start),
                    EK::UnexpectedEndOfExpression,
                ))?;
            }
        };

        let (l_bp, l_ret) = if marker == 0 {
            (0, None)
        } else {
            match ts.get(marker - 1) {
                Some(l_ret) => match binding_power_of(l_ret, current) {
                    None => (0, None),
                    Some((bp, l_ret)) => (bp, Some(l_ret)),
                },
                None => (0, None),
            }
        };
        let (r_bp, r_ret) = match ts.get(marker + 1) {
            Some(r_tok) => match binding_power_of(current, r_tok) {
                None => (0, None),
                Some((bp, r_ret)) => (bp, Some(r_ret)),
            },
            None => (0, None),
        };

        dbg!(l_bp, r_bp);

        // Translated talqual from the bundagerth paper
        if l_bp > r_bp {
            marker += 1;
        } else if l_bp == r_bp {
            if l_bp == 0 {
                todo!("SYNTAX ERROR")
            } else {
                marker += 1;
            }
        } else if l_bp < r_bp {
            // reduce current with left and repeat
            let r_ret = r_ret.unwrap().clone();
            ts.remove(marker);
            //ts.remove(marker - 1);
            ts.insert(marker, r_ret);
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
        alpha: Tree<'src>,
        verb: Tree<'src>,
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
    ts: &'src [ExprToken],
    start_span: Sp<'src>,
) -> TResult<'src, ExprTree<'src>> {
    if ts.is_empty() {
        return Err(TError {
            span: start_span,
            kind: TErrorKind::EmptyExpr,
        });
    }
    parse_expr_go(ts.iter().map(|t| t.to_tree()).collect(), start_span)
}

impl<'src> ExprToken<'src> {
    fn to_tree(&self) -> ExprTree {
        let leaf = |cat| ExprTree::Leaf {
            cat,
            t: self.clone(),
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
    fn span(&self) -> Sp<'src> {
        match self {
            ExprTree::Leaf { cat, t } => t.span,
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
