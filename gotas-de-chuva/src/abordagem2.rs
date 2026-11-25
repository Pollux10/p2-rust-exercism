pub const NOME: &str = "Abordagem 2 (funcional, com tabela e iteradores)";

const SONS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let resultado: String = SONS
        .iter()
        .filter(|&&(divisor, _)| n % divisor == 0)
        .map(|&(_, som)| som)
        .collect();

    if resultado.is_empty() {
        n.to_string()
    } else {
        resultado
    }
}
