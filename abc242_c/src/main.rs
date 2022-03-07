use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut dp = vec![vec![0_i128; 9]; n];
    let a = 998244353;
    for i in 0..9 {
        dp[0_usize][i] = 1;
    }
    for i in 1..n {
        for k in 0..9 {
            if k + 1 == 1 {
                dp[i][k] = (dp[i - 1][0] + dp[i - 1][1]) % a;
            } else if k + 1 == 9 {
                dp[i][k] = (dp[i - 1][7] + dp[i - 1][8]) % a;
            } else {
                dp[i][k] = (dp[i - 1][k - 1] + dp[i - 1][k] + dp[i - 1][k + 1]) % a;
            }
        }
    }

    let count: i128 = dp[n - 1].iter().sum();
    println!("{}", count % 998244353);
}
