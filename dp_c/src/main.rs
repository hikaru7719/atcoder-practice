use proconio::input;

fn main() {
    input! {
        n:usize,
        z: [(u64, u64, u64);n],
    }

    let mut dp = vec![0_u64; n];
    let mut last = -1;
    let (a, b, c) = z[0];
    if a >= b && a >= c {
        dp[0] = a;
        last = 0;
    } else if b >= a && b >= c {
        dp[0] = b;
        last = 1;
    } else if c >= b && c >= a {
        dp[0] = c;
        last = 2;
    }
    for (i, (a, b, c)) in z.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if last == 0 {
            if *b > *c {
                dp[i] = dp[i - 1] + *b;
                last = 1;
            } else {
                dp[i] = dp[i - 1] + *c;
                last = 2;
            }
        } else if last == 1 {
            if *a > *c {
                dp[i] = dp[i - 1] + *a;
                last = 0;
            } else {
                dp[i] = dp[i - 1] + *c;
                last = 2;
            }
        } else if last == 2 {
            if *a > *b {
                dp[i] = dp[i - 1] + *a;
                last = 0;
            } else {
                dp[i] = dp[i - 1] + *b;
                last = 1;
            }
        }
    }
    println!("{}", dp[n - 1]);
}
