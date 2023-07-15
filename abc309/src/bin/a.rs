use proconio::input;

fn main() {
    input! { 
        a: usize,
        b: usize,
     }
     println!("{}", if a % 3 == 0 || a + 1 != b {
        "No"
    } else {
        "Yes"
    })
}
