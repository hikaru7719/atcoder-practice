use proconio::input;

fn main() {
    input! {
        a:f64,
        b:f64,
    }
    let res = ((a.powi(2) + b.powi(2)) as f64).sqrt();

    println!("{} {}", a as f64 / res, b as f64 / res);
}
