pub fn raindrops(n: u32) -> String {
    let mut resultado = String::new();

    if n % 3 == 0 {
        resultado.push_str("Pling");
    }
    if n % 5 == 0 {
        resultado.push_str("Plang");
    }
    if n % 7 == 0 {
        resultado.push_str("Plong");
    }

    if resultado.is_empty() {
        n.to_string()
    } else {
        resultado
    }
}