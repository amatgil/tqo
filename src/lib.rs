use std::{collections::HashMap, convert::Infallible, default};
mod ast;

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
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TFnMonadic(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TFnDyadic(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TModMonadic(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TModDyadic(Infallible);
#[rustfmt::skip] #[derive(Clone, Debug, PartialEq, Eq)] struct TSumType(Infallible);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ident(String);

#[derive(Clone, Debug, PartialEq, Eq)]
enum TAtom {
    /// Arbitrary precision natural (including zero)
    Nat(TNat),
    /// Arbitrary precision integer
    Int(TInt),
    U8(TU8),
    U16(TU16),
    I8(TI8),
    I16(TU16),
    Char(TChar),
    FnMonadic(TFnMonadic),
    FnDyadic(TFnDyadic),
    ModMonadic(TModMonadic),
    ModDyadic(TModDyadic),
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
    FnMonadic,
    FnDyadic,
    ModMonadic,
    ModDyadic,
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum TVerb {
    FnMonadic(TFnMonadic),
    FnDyadic(TFnDyadic),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TAdverb {
    ModMonadic(TModMonadic),
    ModDyadic(TModDyadic),
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

//enum TShapeDescription {
//    Rank(usize),                                // We know the number of axis
//    Axes(Vec<(Option<String>, Option<usize>)>), // We may know the size or name of each axis. Implies rank
//}

//struct TType {
//    scalar_type: TScalarType, // The type
//    shape: TShapeDescription, // And the shape
//}

#[test]
fn bunda_gerth_binding_powers() {
    // See README.md/Bunda-Gerth for the actual table
    let tbd = None; // for now!
    let table: [[Option<u8>; 10]; 10] = [
        /*A*/ [None, tbd, None, tbd, None, tbd, tbd, tbd, tbd, tbd],
        /*αMF*/ [None, tbd, None, tbd, None, tbd, tbd, tbd, tbd, tbd],
        /*⍵MF*/ [tbd, None, tbd, None, tbd, tbd, None, None, None, tbd],
        /*DF*/ [tbd, None, tbd, None, tbd, tbd, None, None, None, tbd],
        /*N*/ [None, tbd, None, tbd, None, tbd, tbd, tbd, tbd, tbd],
        /*αMM*/ [None, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd],
        /*⍵MM*/ [tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd],
        /*DM*/ [tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd],
        /*JOT*/ [tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd],
        /*ARR*/ [tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd, tbd],
    ];

    // This is ordered
    #[repr(u8)]
    enum Ty {
        A,
        A_MF,
        O_MF,
        DF,
        N,
        A_MM,
        O_MM,
        DM,
        JOT,
        Arr,
    }
    use Ty::*;

    let gt_conditions = [
        ((A_MF, DM), (DM, DF)),
        ((DF, DM), (DM, DF)),
        ((DF, JOT), (DM, DF)),
    ];

    for (lesser, greater) in gt_conditions {
        let lesser_prio = table[lesser.0 as u8 as usize][lesser.1 as u8 as usize];
        let greater_prio = table[greater.0 as u8 as usize][greater.1 as u8 as usize];

        assert!(lesser_prio.is_none() || greater_prio.unwrap() > lesser_prio.unwrap())
    }
}
