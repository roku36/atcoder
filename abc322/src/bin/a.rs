use proconio::input;

fn main() {
    // input int n and n-length string s
    input! {
        _n: usize,
        s: String,
    }

    // if s have substring 'abc' print the index, else print -1
    if s.contains("ABC") {
        println!("{}", s.find("ABC").unwrap()+1);
    } else {
        println!("-1");
    }
}
