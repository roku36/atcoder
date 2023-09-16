use proconio::input;

fn main() {
    // input u32, and strings s1, s2, s3
    input! {
        m: u32, // length of these strings
        s1: String,
        s2: String,
        s3: String,
    }

    // convert strings to array of numbers
    let s1: Vec<u32> = s1.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let s2: Vec<u32> = s2.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let s3: Vec<u32> = s3.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // boolean flag
    let mut have_shared_digit = false;

    // for each 0~9, if those 3 strings contains it, print
    for i in 0..10 {
        if s1.contains(&i) && s2.contains(&i) && s3.contains(&i) {
            have_shared_digit = true;
        }
        
    }

    // if no shared number, break.
    if !have_shared_digit {
        println!("No");
    } else {
        
    }
}
