use proconio::input;

fn main() {
    input! {
        n: usize,
        p_vec: [i32; n],
    }

    let mut p_max = *p_vec.iter().max().unwrap();

    // set boolean `max_exist` to true if p_max is equal to p_vec[1~n]
    let mut max_exist = false;
    for i in 1..n {
        if p_vec[i] == p_max {
            max_exist = true;
            break;
        }
    }

    let max_exist_alpha = max_exist as i32;

    // print p_max - p_vec[0] if p_vec[0] != p_max
    println!("{}", if p_vec[0] != p_max { p_max - p_vec[0] + 1 } else { 0 + max_exist_alpha });
}
