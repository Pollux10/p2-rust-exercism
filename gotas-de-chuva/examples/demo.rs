use sublist::{sublist, NOME};

fn main() {
    println!("Abordagem ativa: {}", NOME);
    println!();

    let casos: [(&[i32], &[i32]); 8] = [
        (&[], &[]),
        (&[1, 2, 3], &[]),
        (&[], &[1, 2, 3]),
        (&[1, 2, 3], &[1, 2, 3, 4, 5]),
        (&[3, 4], &[1, 2, 3, 4, 5]),
        (&[1, 2, 3, 4, 5], &[2, 3, 4]),
        (&[1, 2, 4], &[1, 2, 3, 4, 5]),
        (&[1, 2, 3], &[1, 3, 2]),
    ];

    for (a, b) in casos {
        println!("A = {:?}   B = {:?}   ->   {:?}", a, b, sublist(a, b));
    }
}
