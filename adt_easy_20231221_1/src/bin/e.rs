use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
      h: usize,
      w: usize,
      a: [[usize; w]; h]
    }

    let mut ans = 0;

    solve(0, 0, HashSet::new(), &a, h, w, &mut ans);

    println!("{}", ans);
}

fn solve(
    i: usize,
    j: usize,
    set: HashSet<usize>,
    a: &Vec<Vec<usize>>,
    h: usize,
    w: usize,
    ans: &mut usize,
) {
    let mut s = set.clone();

    if s.get(&a[i][j]).is_none() {
        s.insert(a[i][j]);
    } else {
        return;
    }

    if i == h - 1 && j == w - 1 {
        *ans += 1;
    }

    if i + 1 < h {
        solve(i + 1, j, s.clone(), a, h, w, ans)
    }

    if j + 1 < w {
        solve(i, j + 1, s.clone(), a, h, w, ans)
    }
}
