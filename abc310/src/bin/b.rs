use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut p = vec![];
    let mut c = vec![];
    let mut func = vec![vec![false; m]; n];
    for i in 0..n {
        input! {
            pi: i64,
            ci: usize,
            f: [Usize1; ci],
        }
        p.push(pi);
        c.push(ci);
        for k in f {
            func[i][k] = true;
        }
    }

    for i in 0..n {
        for j in (0..n).filter(|&j| i != j) {
            if p[i] >= p[j]
                && (0..m).filter(|k| func[i][*k]).all(|k| func[j][k])
                && (p[i] > p[j] || c[i] < c[j])
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
