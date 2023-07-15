use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut count = vec![0; n + 1];
    let mut sum = 0;
    let mut total = 0;
    let s = s.chars().collect::<Vec<char>>();

    for i in 0..n {
        if s[i] == '1' {
            sum += 1;
            total += sum;
            count[sum] += 1;
        } else {
            sum = 0;
        }
    }

    let mut result = 0;
    for i in 0..n {
        if s[i] == '1' {
            result += n - i;
            let decrement = count[sum]
                * (sum
                    .checked_sub(total.checked_sub(sum).unwrap_or(0))
                    .unwrap_or(0));
            result = result.checked_sub(decrement).unwrap_or(0);
            total = total.checked_sub(sum).unwrap_or(0);
            sum = sum.checked_sub(1).unwrap_or(0);
        }
    }

    println!("{}", result);
}
