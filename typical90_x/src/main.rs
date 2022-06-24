use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i128,
        a: [i128; n],
        b: [i128; n],
    }
    let v: i128 = a.iter().enumerate().map(|(n, aa)| (aa - b[n]).abs()).sum();

    if k < v {
        println!("No");
    } else if (k - v) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
