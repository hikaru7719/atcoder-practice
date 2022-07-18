use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    };
    let z = PI / 180_f64 * c;
    println!(
        "{} {}",
        a * z.cos() - b * z.sin(),
        a * z.sin() + b * z.cos()
    );
}
