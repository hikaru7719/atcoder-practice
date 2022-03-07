use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        b: [(i64, i64); n],
    }
    let mut dp = vec![vec![-1_i64 << 31; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..n + 1 {
        let (ww, v) = b[i - 1];
        for k in 0..w + 1 {
            if 0 <= k as i64 - ww {
                dp[i][k as usize] =
                    std::cmp::max(dp[i - 1][k as usize], dp[i - 1][(k - ww as usize)] + v);
            } else {
                dp[i][k as usize] = dp[i - 1][k as usize];
            }
        }
    }

    let mut max = -1;
    for m in dp[n].iter() {
        max = std::cmp::max(max, *m);
    }
    println!("{}", max);
}
