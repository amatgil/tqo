use std::{collections::HashMap, convert::Infallible, default};

use ast::Sp;
mod ast;
mod function;
mod parsing;
mod primitive;

#[derive(Debug, Clone, Copy)]
struct TError<'src> {
    span: Sp<'src>,
    kind: TErrorKind,
}

impl<'src> TError<'src> {
    pub fn new(kind: TErrorKind, span: Sp<'src>) -> Self {
        Self { span, kind }
    }
}

#[derive(Debug, Clone, Copy)]
enum TErrorKind {
    EmptyExpr,
}

type TResult<'src, T> = Result<T, TError<'src>>;

enum Side {
    Left,
    Right,
}

#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TNat(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TInt(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TU8(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TU16(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TI8(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TI16(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TChar(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TAVerb(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TOVerb(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TDVerb(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TAAdverb(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TOAdverb(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TDAdverb(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TSumType(Infallible);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ident(String);

#[derive(Clone, Debug, PartialEq, Eq)]
enum TAtom {
    /// Arbitrary precision natural (including zero)
    Nat(TNat),
    /// Arbitrary precision integer
    Int(TInt),
    /// Unsigned eight-bit number
    U8(TU8),
    /// Unsigned sixteen-bit number
    U16(TU16),
    /// Signed eight-bit number
    I8(TI8),
    /// Signed sixteen-bit number
    I16(TU16),
    /// Unicode character
    Char(TChar),
    /// alpha-monadic verb
    AVerb(TAVerb),
    /// omega-monadic verb
    OVerb(TOVerb),
    /// alpha-monadic adverb
    AAdverb(TAVerb),
    /// omega-monadic adverb
    OAdverb(TOVerb),
    SumType(TSumType),
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum TAtomKind {
    /// Arbitrary precision natural (including zero)
    Nat,
    /// Arbitrary precision integer
    Int,
    U8,
    U16,
    I8,
    I16,
    Char,
    AVerb,
    OVerb,
    DVerb,
    AAdverb,
    OAdverb,
    DDyadic,
    SumType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TArray {
    shape: Vec<u64>, // TODO: Turn this into a tinyvec
    data: Vec<TNoun>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TNoun {
    Atom(TAtom),
    Array(TArray),
}

#[derive(Clone, Debug)]
struct TEnv(HashMap<Ident, TNoun>);

impl Default for TEnv {
    fn default() -> Self {
        let hs = HashMap::from([(1, 2)]);
        todo!()
    }
}

enum Primitive {
    Negate,
    Not,
    Sign,
    Add,
    Subtract,
    Multiply,
    Divide,
    AbsoluteValue,
    Sine,
    Reciprocal,
    Sqrt,
    Modulo,
    Maximum,
    Minimum,
}
