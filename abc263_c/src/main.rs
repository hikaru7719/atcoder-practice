use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    for mut v in dfs(0, n, 0, m) {
        v.pop();
        v.reverse();
        let ans = v
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", ans);
    }
}

fn dfs(i: usize, n: usize, j: usize, m: usize) -> Vec<Vec<usize>> {
    if i == n {
        return vec![vec![j]];
    }
    let mut res = vec![];
    for nj in j + 1..=m {
        for mut v in dfs(i + 1, n, nj, m) {
            v.push(j);
            res.push(v);
        }
    }
    res
}
