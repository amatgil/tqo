use std::{convert::Infallible, marker::PhantomData};

use crate::{Ident, Primitive, TAtomKind};

enum TypeScalarAnnotation {
    Base(TAtomKind),
    Rec(TypeAnnotation),
}
struct TypeAnnotation {
    scalar_type: Box<TypeScalarAnnotation>, // The type
    shape: ShapeDescription,                // And the shape
}

/// TODO: Indicate properly, as a struct, that we may know rank and length and such
enum ShapeDescription {
    Rank(usize),                                // We know the number of axis
    Axes(Vec<(Option<String>, Option<usize>)>), // We may know the size or name of each axis. Implies rank
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sp<'src> {
    /// In bytes
    pub start: usize,
    /// In bytes also (non-inclusive)
    pub end: usize,
    /// Lifetime holder
    _phantom: PhantomData<&'src ()>,
}

impl<'a> Sp<'a> {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            _phantom: PhantomData,
        }
    }
    pub fn merge<'src>(&'src self, rhs: &'src Sp) -> Sp<'src> {
        Self::new(self.start.min(rhs.start), self.start.max(rhs.start))
    }
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
pub enum Item<'src> {
    TypeDef {
        left: Option<TypeAnnotation>,
        right: Option<TypeAnnotation>,
        out: Option<TypeAnnotation>,
    },
    Binding {
        name: Ident,
        code: Vec<(Word, Sp<'src>)>,
    },
    Words(Vec<(Word, Sp<'src>)>),
}
