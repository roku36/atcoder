use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut data = Vec::new();

    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c],
        }
        data.push((p, c, f));
    }
    for i in 0..n {
        let (p, c, f) = &data[i];
        println!("P: {}, C: {}", p, c);
        for j in 0..*c {
            println!("F[{}]: {}", j + 1, f[j]);
        }
    }
}
