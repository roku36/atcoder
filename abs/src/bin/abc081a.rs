use proconio::input;

fn main() {
    input! {
        s: String,
    }

    // check how many digits have "5"
    let mut count = 0;
    for c in s.chars() {
        if c == '1' {
            count += 1;
        }
    }
    println!("{}", count);
}
