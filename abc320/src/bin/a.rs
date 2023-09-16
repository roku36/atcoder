use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    // print a^b + b^a
    println!("{}", a.pow(b) + b.pow(a));
}
