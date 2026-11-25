#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub mod abordagem1;
pub mod abordagem2;

// Para testar a abordagem 1, deixe esta linha ativa:
pub use abordagem1::{sublist, NOME};

// Para testar a abordagem 2, comente a linha acima e ative esta:
//pub use abordagem2::{sublist, NOME};
