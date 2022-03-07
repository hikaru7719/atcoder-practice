use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        v:[(usize, usize); n]
    }
    let mut dp = vec![vec![0; x + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..n + 1 {
        for k in 0..x + 1 {
            if 0_i32 <= k as i32 - v[i - 1].0 as i32 && dp[i - 1][k - v[i - 1].0] == 1 {
                dp[i][k] = 1;
            }
            if 0_i32 <= k as i32 - v[i - 1].1 as i32 && dp[i - 1][k - v[i - 1].1] == 1 {
                dp[i][k] = 1;
            }
        }
    }
    if dp[n][x] == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
