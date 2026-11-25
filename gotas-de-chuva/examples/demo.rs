use raindrops::{raindrops, NOME};

fn main() {
    println!("Abordagem ativa: {}", NOME);
    println!();
    println!("{:<6} | {}", "n", "resultado");
    println!("{}", "-".repeat(30));

    for n in [1, 3, 5, 7, 8, 15, 21, 28, 30, 34, 35, 105] {
        println!("{:<6} | {}", n, raindrops(n));
    }
}
