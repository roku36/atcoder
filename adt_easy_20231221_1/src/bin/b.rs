use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", s.chars().position(|c| c.is_uppercase()).unwrap() + 1);
}
