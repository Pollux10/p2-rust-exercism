use crate::Comparison;
use std::cmp::Ordering;

pub const NOME: &str = "Abordagem 2 (comparação de tamanhos e busca manual)";

pub fn sublist<T: PartialEq>(primeira: &[T], segunda: &[T]) -> Comparison {
    match primeira.len().cmp(&segunda.len()) {
        Ordering::Equal => {
            if primeira == segunda {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if contem(primeira, segunda) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if contem(segunda, primeira) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn contem<T: PartialEq>(maior: &[T], menor: &[T]) -> bool {
    for inicio in 0..=(maior.len() - menor.len()) {
        if maior[inicio..].starts_with(menor) {
            return true;
        }
    }
    false
}
