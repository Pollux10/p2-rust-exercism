use crate::Comparison;

pub fn sublist<T: PartialEq>(primeira: &[T], segunda: &[T]) -> Comparison {
    if primeira == segunda {
        Comparison::Equal
    } else if contem(primeira, segunda) {
        Comparison::Superlist
    } else if contem(segunda, primeira) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contem<T: PartialEq>(maior: &[T], menor: &[T]) -> bool {
    menor.is_empty() || maior.windows(menor.len()).any(|janela| janela == menor)
}