use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    }

    if (r + c <= 16 && r >= c) || (r + c >= 16 && r <= c) {
        // println!("1st");
        if c % 2 == 0 {
            println!("white");
        } else {
            println!("black");
        }
    } else {
        // println!("2st");
        if r % 2 == 0 {
            println!("white");
        } else {
            println!("black");
        }
    }
}
