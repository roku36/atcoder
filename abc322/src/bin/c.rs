use proconio::input;

fn main() {
    // input int n,m and int array A1 .. Am
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut map = vec![-1; n];
    for i in 0..m {
        map[a[i]-1] = 0;
    }

    for i in (0..n).rev() {
        if map[i] == -1 {
            map[i] = map[i+1] + 1;
        }
    }

    for i in 0..n {
        println!("{}", map[i]);
    }
}
