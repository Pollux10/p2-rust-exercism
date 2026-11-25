#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub mod abordagem1;
pub mod abordagem2;

pub use abordagem1::sublist;
//pub use abordagem2::sublist;