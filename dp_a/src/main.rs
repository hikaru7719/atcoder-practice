use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut dp: Vec<i32> = vec![1 << 20; n];
    dp[0] = 0;
    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i - 1] - h[i]).abs();
        } else {
            dp[i] = std::cmp::min(
                dp[i - 1] + (h[i - 1] - h[i]).abs(),
                dp[i - 2] + (h[i - 2] - h[i]).abs(),
            );
        }
    }
    println!("{}", dp[n - 1]);
}
