use raindrops::{abordagem1, abordagem2};

fn main() {
    println!("{:<6} | {:<16} | {:<16} | {}", "n", "abordagem 1", "abordagem 2", "iguais?");
    println!("{}", "-".repeat(55));

    for n in [1, 3, 5, 7, 8, 15, 21, 28, 30, 34, 35, 105] {
        let r1 = abordagem1::raindrops(n);
        let r2 = abordagem2::raindrops(n);
        let iguais = if r1 == r2 { "OK" } else { "DIFERENTE" };
        println!("{:<6} | {:<16} | {:<16} | {}", n, r1, r2, iguais);
    }
}