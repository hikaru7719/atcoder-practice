use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut sum = 0_i64;
    for &a_i in a.iter() {
        sum += a_i.pow(2) * (n as i64 - 1);
    }

    let mut c = 0;
    for i in (1..n).rev() {
        c += a[i];
        sum -= 2 * a[i - 1] * c;
    }

    let ans = sum;
    println!("{}", ans);
}
