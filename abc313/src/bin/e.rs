use proconio::input;

const MOD: u64 = 998244353;

fn replace_s(s: &str) -> i64 {
    let mut t = String::new();
    let mut count = 0;

    let mut s_chars: Vec<char> = s.chars().collect();
    while s_chars.len() > 1 {
        let len = s_chars.len();
        let n: usize = s_chars[len - 1].to_digit(10).unwrap() as usize;
        t.push_str(&s_chars[..len - 1].iter().collect::<String>().repeat(n));
        count += 1;

        s_chars = t.chars().collect();
        t.clear();

        if count > 1000000 {
            return -1;
        }
    }

    count % (MOD as i64)
}

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let result = replace_s(&s);
    println!("{}", result);
}
