use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [isize; n],
    }

    for i in 0..n {
        if i == 0 {
            print!("{}", s[i]);
        } else {
            print!(" {}", s[i] - s[i - 1]);
        }
    }
    println!();
}
