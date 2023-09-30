use proconio::input;

struct Dev {
    c: usize, // cost
    a: Vec<usize>, // improvement
}

fn main() {
    // input int k,n,p
    // c1 a11 a12 .. a1k
    // c1 a21 a22 .. a2k
    // ...
    // cn an1 an2 .. ank
    input! {
        k: usize, // param number
        n: usize, // dev number
        p: usize, // goal of improvement
        data: [(usize, [usize; k+1]); n]
    }

    let devs: Vec<Dev> = data.into_iter().map(|(c, a)| Dev { c, a }).collect();

    // dp table p*k, initialized with INF
    let mut dp = vec![vec![std::usize::MAX; p+1]; k+1];
    // let if p is 0 initialized with 0
    for i in 0..=k {
        dp[i][0] = 0;
    }

    // update dp table
    for i in 0..n {
        for j in (0..=k).rev() {
            for l in (0..=p).rev() {
                for m in 0..=j {
                    if m <= devs[i].c && l >= devs[i].a[m] && dp[j][l] > dp[j-m][l-devs[i].a[m]] + devs[i].c {
                        dp[j][l] = dp[j-m][l-devs[i].a[m]] + devs[i].c;
                    }
                }
            }
        }
    }

    // print each dp[p] 
    for i in 0..=k {
        println!("{}", dp[i][p]);
    }
}
