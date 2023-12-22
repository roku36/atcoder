use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    // a*b even or odd
    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
