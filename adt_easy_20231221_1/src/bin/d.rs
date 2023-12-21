use proconio::input;

fn main() {
    input! {
        n: isize,
    }
    let modulus = 998244353;
    let ans = {
        if n >= 0 {
            n % modulus
        } else {
            let x = ((-n + modulus - 1) / modulus) * modulus + n;
            x.abs()
        }
    };

    println!("{}", ans);
}
