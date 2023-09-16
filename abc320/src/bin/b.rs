use proconio::input;

fn main() {
    // input string s
    input! {
        s: String,
    }

    // int variable maximum_length
    let mut maximum_length = 0;

    // search all substrings
    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            // if substring is palindrome
            if s[i..j].chars().rev().collect::<String>() == s[i..j].chars().collect::<String>() {
                // update maximum_length
                if j - i > maximum_length {
                    maximum_length = j - i;
                }
            }
            
        }
    }

    // print maximum_length
    println!("{}", maximum_length);
}
