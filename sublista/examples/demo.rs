use sublist::{abordagem1, abordagem2};

fn main() {
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
        let r1 = abordagem1::sublist(a, b);
        let r2 = abordagem2::sublist(a, b);
        let iguais = if r1 == r2 { "OK" } else { "DIFERENTE" };

        println!("A = {:?}   B = {:?}", a, b);
        println!("   abordagem 1: {:?} | abordagem 2: {:?} | {}", r1, r2, iguais);
        println!();
    }
}