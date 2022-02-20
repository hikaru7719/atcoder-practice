use proconio::input;

fn main() {
    input! {
        h: f64
    }
    let a = 12800000_f64 * h;
    let b = h.powi(2);
    println!("{}", (a + b).sqrt());
}
