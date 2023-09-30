use proconio::input;

fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: String,
        t: String,
    }

    // mutex int value answer
    let mut ans = 0;

    // if s is not suffix of t
    if !t.ends_with(&s) {
        ans += 1;
    }

    if !t.starts_with(&s) {
        ans += 2;
    }

    println!("{}", ans);
}
