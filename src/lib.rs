use std::convert::Infallible;

enum Side {
    Left,
    Right,
}

/// Elements of depth 0
enum TAtom {
    Nat(Infallible), // TODO
    Int(Infallible), // TODO
    U8(u8),
    U16(u16), // TODO: add the rest of the unsigned ones
    I8(u8),
    I16(u16), // TODO: add the rest of the signed ones
    Char(char),
    FunctionMonadic {},
    FunctionDyadic { side: Side },
    ModifierMonadic,
    ModifierDyadic,
    SumType(Infallible),
}

struct TArray {
    shape: Vec<u64>, // TODO: Turn this into a tinyvec
    data: Vec<TValue>,
}

enum TValue {
    Atom(TAtom),
    Array(TArray),
}

enum TScalarType {
    Nat,
    Int,
    Float,
    Char,
    // TODO: add sum types
}

//enum TShapeDescription {
//    Rank(usize),                                // We know the number of axis
//    Axes(Vec<(Option<String>, Option<usize>)>), // We may know the size or name of each axis. Implies rank
//}

//struct TType {
//    scalar_type: TScalarType, // The type
//    shape: TShapeDescription, // And the shape
//}
