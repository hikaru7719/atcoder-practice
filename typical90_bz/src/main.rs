use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(usize, usize);m],
    };
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    v.into_iter().for_each(|(a, b)| {
        graph[(a - 1) as usize].push(b - 1);
        graph[(b - 1) as usize].push(a - 1);
    });

    let result = graph
        .into_iter()
        .enumerate()
        .map(|(i, node)| {
            if node.into_iter().filter(|n| *n < i).count() == 1 {
                1
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{}", result);
}
