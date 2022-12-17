mod tree;
mod formula;
mod atom;
mod eval;

pub use formula::Formula;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LogicType {
    Unary,
    Binary,
    NestOpen,
    NestClose,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ElementType {
    Unary,
    Binary(char),
    NestOpen,
    NestClose,
    Atom(char),
}
