use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut some = 0;
    for i in 0..n {
        if a[i] - 1 == i as i64 {
            some += 1;
        }
    }
    let mut ans: i128 = some * (some - 1) / 2;

    for i in 0..n {
        if a[i] - 1 > i as i64 && a[i] - 1 < n as i64 && a[(a[i] - 1) as usize] - 1 == i as i64 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
