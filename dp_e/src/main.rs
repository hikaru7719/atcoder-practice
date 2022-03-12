use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        b: [(i64, i64); n],
    }
    let mut dp = vec![vec![1_i64 << 31; 10_i32.pow(5) as usize]; n + 1];
    dp[0][0] = 0;
    for i in 1..n + 1 {
        let (ww, v) = b[i - 1];
        for k in 0..10_i32.pow(5) as usize {
            if 0 <= k as i64 - v {
                dp[i][k as usize] = dp[i][k - v as usize] + ww;
            } else {
                dp[i][k as usize] = dp[i - 1][k as usize];
            }
        }
    }

    let mut max = 0;
    for m in dp.iter() {
        for (v, l) in m.iter().enumerate() {
            if *l <= w as i64 {
                max = std::cmp::max(max, v);
            }
        }
    }
    println!("{}", max);
}
