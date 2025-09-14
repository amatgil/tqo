use std::convert::Infallible;

use crate::{Ident, Primitive, TAtomKind};

enum TypeScalarAnnotation {
    Base(TAtomKind),
    Rec(TypeAnnotation),
}
struct TypeAnnotation {
    scalar_type: Box<TypeScalarAnnotation>, // The type
    shape: ShapeDescription,                // And the shape
}

enum ShapeDescription {
    Rank(usize),                                // We know the number of axis
    Axes(Vec<(Option<String>, Option<usize>)>), // We may know the size or name of each axis. Implies rank
}

#[derive(Clone, Copy)]
pub struct Sp<T> {
    /// In bytes
    pub start: usize,
    /// In bytes also
    pub end: usize,
    /// The actual thing
    pub value: T,
}

/// A word, directly as read in
pub enum Word {
    // Nat -> Int -> Float
    Number(String),
    String(String),
    Array(Infallible),
    Primitive(Primitive),
}

/// What appears in the code, pretty much at any location
pub enum Item {
    TypeDef {
        left: Option<TypeAnnotation>,
        right: Option<TypeAnnotation>,
        out: Option<TypeAnnotation>,
    },
    Binding {
        name: Sp<Ident>,
        code: Vec<Sp<Word>>,
    },
    Words(Vec<Sp<Word>>),
}
