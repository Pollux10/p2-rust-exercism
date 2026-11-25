use sublist::{sublist, Comparison};

#[test]
fn listas_vazias() {
    let a: &[i32] = &[];
    let b: &[i32] = &[];
    assert_eq!(sublist(a, b), Comparison::Equal);
}

#[test]
fn lista_vazia_dentro_de_lista_nao_vazia() {
    let a: &[i32] = &[];
    assert_eq!(sublist(a, &[1, 2, 3]), Comparison::Sublist);
}

#[test]
fn lista_nao_vazia_contem_lista_vazia() {
    let b: &[i32] = &[];
    assert_eq!(sublist(&[1, 2, 3], b), Comparison::Superlist);
}

#[test]
fn lista_igual_a_ela_mesma() {
    assert_eq!(sublist(&[1, 2, 3], &[1, 2, 3]), Comparison::Equal);
}

#[test]
fn listas_diferentes() {
    assert_eq!(sublist(&[1, 2, 3], &[2, 3, 4]), Comparison::Unequal);
}

#[test]
fn falso_inicio() {
    assert_eq!(sublist(&[1, 2, 5], &[0, 1, 2, 3, 1, 2, 5, 6]), Comparison::Sublist);
}

#[test]
fn elementos_consecutivos() {
    assert_eq!(sublist(&[1, 1, 2], &[0, 1, 1, 1, 2, 1, 2]), Comparison::Sublist);
}

#[test]
fn sublista_no_inicio() {
    assert_eq!(sublist(&[0, 1, 2], &[0, 1, 2, 3, 4, 5]), Comparison::Sublist);
}

#[test]
fn sublista_no_meio() {
    assert_eq!(sublist(&[2, 3, 4], &[0, 1, 2, 3, 4, 5]), Comparison::Sublist);
}

#[test]
fn sublista_no_fim() {
    assert_eq!(sublist(&[3, 4, 5], &[0, 1, 2, 3, 4, 5]), Comparison::Sublist);
}

#[test]
fn superlista_no_inicio() {
    assert_eq!(sublist(&[0, 1, 2, 3, 4, 5], &[0, 1, 2]), Comparison::Superlist);
}

#[test]
fn superlista_no_meio() {
    assert_eq!(sublist(&[0, 1, 2, 3, 4, 5], &[2, 3]), Comparison::Superlist);
}

#[test]
fn superlista_no_fim() {
    assert_eq!(sublist(&[0, 1, 2, 3, 4, 5], &[3, 4, 5]), Comparison::Superlist);
}

#[test]
fn primeira_lista_sem_elemento_da_segunda() {
    assert_eq!(sublist(&[1, 3], &[1, 2, 3]), Comparison::Unequal);
}

#[test]
fn segunda_lista_sem_elemento_da_primeira() {
    assert_eq!(sublist(&[1, 2, 3], &[1, 3]), Comparison::Unequal);
}

#[test]
fn digitos_adicionais_faltando() {
    assert_eq!(sublist(&[1, 2], &[1, 22]), Comparison::Unequal);
}

#[test]
fn ordem_importa() {
    assert_eq!(sublist(&[1, 2, 3], &[3, 2, 1]), Comparison::Unequal);
}

#[test]
fn mesmos_digitos_numeros_diferentes() {
    assert_eq!(sublist(&[1, 0, 1], &[10, 1]), Comparison::Unequal);
}