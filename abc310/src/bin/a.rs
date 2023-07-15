use proconio::input;

fn main() {
    input! {
        n: i32,
        p: i32,
        q: i32,
        d_vec: [i32; n],
    }

    let min_d = d_vec.iter().min().unwrap();

    // print smaller one of (q, p+d_min)
    println!("{}", if p < q + min_d { p } else { q + min_d });
}
