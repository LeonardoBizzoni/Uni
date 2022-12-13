mod tree;
mod formula;

pub use formula::Formula;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LogicOperator {
    And,
    Or,
    Not,
    Imp,
    Iff,
    NestOpen,
    NestClose,
}
