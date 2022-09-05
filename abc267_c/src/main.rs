use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut sum = vec![0_i64; n];
    sum[0] = a[0];
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i];
    }

    let mut result: i64 = -1000000000000000000;
    for i in 0..a.len() {
        if let Some(x) = calc(i, m, &a) {
            result = std::cmp::max(result, x);
        } else {
            break;
        }
    }
    println!("{}", result);
}

fn calc(i: usize, m: usize, a: &Vec<i64>) -> Option<i64> {
    if i + m - 1 < a.len() {
        let mut result: i64 = 0;
        for (index, ii) in (i..=i + m - 1).enumerate() {
            result += (index + 1) as i64 * a[ii];
        }
        return Some(result);
    }
    None
}
