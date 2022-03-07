use proconio::input;

fn main() {
    input! {
        n:usize,
        z: [(u64, u64, u64);n],
    }

    let mut dp = vec![vec![0_u64; 3]; n];
    dp[0][0] = z[0].0;
    dp[0][1] = z[0].1;
    dp[0][2] = z[0].2;
    for i in 1..n {
        dp[i][0] = std::cmp::max(dp[i - 1][1], dp[i - 1][2]) + z[i].0;
        dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][2]) + z[i].1;
        dp[i][2] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]) + z[i].2;
    }
    println!(
        "{}",
        std::cmp::max(std::cmp::max(dp[n - 1][0], dp[n - 1][1]), dp[n - 1][2])
    );
}
