use crate::*;

struct TypeCheckErr<'src> {
    span: Sp<'src>,
    kind: TypeCheckErrKind,
}
enum TypeCheckErrKind {
    InternalParsingErr,
}

pub fn typecheck_token<'src>(
    s: &'src str,
    t: ExprToken<'src>,
) -> Result<TType, TypeCheckErr<'src>> {
    use InputTypeSpecifier as ITS;
    use OutputTypeSpecifier as OTS;
    match t.kind {
        ExprTokenKind::Number | ExprTokenKind::Char => Ok(TType {
            alpha: None,
            omega: None,
            output: Some(OTS {
                rank: Some(Box::new(|_, _| 0)),
                length: Some(Box::new(|_, _| 1)),
                shape: Some(Box::new(|_, _| vec![])),
            }),
        }),
        ExprTokenKind::String => Ok(TType {
            alpha: None,
            omega: None,
            output: Some(OTS {
                rank: Some(Box::new(|_, _| 1)),
                length: Some(Box::new(|_, _| todo!())),
                shape: Some(Box::new(|_, _| todo!())),
            }),
        }),
        ExprTokenKind::PrimArray => todo!(),
        ExprTokenKind::ArrayName => todo!(),
        ExprTokenKind::PrimAVerb => todo!(),
        ExprTokenKind::PrimOVerb => todo!(),
        ExprTokenKind::PrimDVerb => todo!(),
        ExprTokenKind::PrimAAdverb => todo!(),
        ExprTokenKind::PrimOAdverb => todo!(),
        ExprTokenKind::PrimDAdverb => todo!(),
        ExprTokenKind::AVerbName => todo!(),
        ExprTokenKind::OVerbName => todo!(),
        ExprTokenKind::DVerbName => todo!(),
        ExprTokenKind::OAdverbName => todo!(),
        ExprTokenKind::DAdverbName => todo!(),
        ExprTokenKind::ArrayAssign => todo!(),
        ExprTokenKind::VerbAssign => todo!(),
        ExprTokenKind::AdverbAssign => todo!(),
        ExprTokenKind::Parenthesized(expr_tokens) => todo!(),
    }
}
pub fn typecheck_expr(e: ExprTree) -> Result<Typedef, TypeCheckErr> {
    match e {
        ExprTree::Leaf { cat, t } => todo!(),
        ExprTree::AlphaVerbCall { verb, alpha } => todo!(),
        ExprTree::OmegaVerbCall { verb, omega } => todo!(),
        ExprTree::DyadicVerbCall { verb, alpha, omega } => todo!(),
        ExprTree::Assignment { name, val } => todo!(),
        ExprTree::AlphaAdverbCall { adverb, alpha } => todo!(),
        ExprTree::OmegaAdverbCall { adverb, omega } => todo!(),
        ExprTree::DyadicAdverbCall {
            adverb,
            alpha,
            omega,
        } => todo!(),
    }
    todo!()
}
