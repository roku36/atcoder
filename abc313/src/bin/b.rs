use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        relationships: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    let mut in_degrees = vec![0; n + 1];

    for (a, b) in relationships {
        graph[a].push(b);
        in_degrees[b] += 1;
    }

    let mut queue = VecDeque::new();
    for i in 1..=n {
        if in_degrees[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut topological_order = Vec::new();
    while let Some(node) = queue.pop_front() {
        topological_order.push(node);

        for &adjacent in &graph[node] {
            in_degrees[adjacent] -= 1;
            if in_degrees[adjacent] == 0 {
                queue.push_back(adjacent);
            }
        }
    }

    if topological_order.len() == n {
        let strongest_programmer = topological_order.last().unwrap();
        let has_relationship_with_all = graph[*strongest_programmer]
            .iter()
            .all(|&adjacent| in_degrees[adjacent] == 0);

        if has_relationship_with_all {
            println!("{}", strongest_programmer);
        } else {
            println!("-1");
        }
    } else {
        println!("-1");
    }
}
