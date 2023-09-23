use proconio::input;

fn main() {
    // input
    // n x
    // A1 A2 ... An-1
    input! {
        n: usize,
        x: usize,
        a: [usize; n-1]
    }

    // get max and min of a
    let max_a = *a.iter().max().unwrap();
    let min_a = *a.iter().min().unwrap();
    let sum = a.iter().sum::<usize>();

    let mut result = 0;

    // for 0 to 100
    for i in 0..=100 {
        // if max_a <= i
        // result = sum - i - min_a
        if max_a <= i {
            result = sum - min_a;
        // elif min > i
        } else if min_a >= i {
            result = sum - max_a;
        } else {
            result = sum + i - max_a - min_a
        }

        // println!("{}", result);
        if result >= x {
            // print i and finish running
            println!("{}", i);
            return
        }
    }

    // print result
    println!("-1");
}
