use proconio::input;
fn main() {
    input! {
        a: f64,
        b: f64
    }
    println!("{}", ((a - b) / a) * 100_f64);
}
