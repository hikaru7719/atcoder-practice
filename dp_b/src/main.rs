use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        h: [i32;n],
    }
    let mut dp: Vec<i32> = vec![0; n];
    for i in 1..n {
        dp[i] = min_jump(k, i, &h, &dp);
    }
    println!("{}", dp[n - 1]);
}

fn min_jump(k: i32, index: usize, h: &Vec<i32>, dp: &Vec<i32>) -> i32 {
    let mut min = 1 << 30;
    for i in 1..k + 1 {
        if 0 <= index as i32 - i {
            min = std::cmp::min(
                min,
                dp[index - i as usize] + (h[index as usize] - h[index - i as usize]).abs(),
            )
        }
    }
    return min;
}
