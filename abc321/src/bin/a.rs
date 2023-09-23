use proconio::input;

fn main() {
    // input string
    input! {
        s: String
    }

    // flag if monotonically decreasing
    let mut decreasing = true;

    // for loop ith and i+1th characters
    for i in 0..s.len()-1 {
        // compare ith and i+1th as number
        let ith = s.chars().nth(i).unwrap().to_digit(10).unwrap();
        let i_plus_1th = s.chars().nth(i+1).unwrap().to_digit(10).unwrap();
        // if ith is bigger than i+1th
        if ith <= i_plus_1th {
            decreasing = false; 
        }
    }

    // print if monotonically decreasing
    println!("{}", if decreasing { "Yes" } else { "No" });
}
