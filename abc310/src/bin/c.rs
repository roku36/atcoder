use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        b: bool,
    }

    let mut set = HashSet::new();

    for string in s {
        let reverse: String = string.chars().rev().collect();

        if !set.contains(&string) && !set.contains(&reverse) {
            set.insert(string);
        }
    }

    println!("{}", set.len());
}
