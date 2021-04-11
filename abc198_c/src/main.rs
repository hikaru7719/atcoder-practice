use proconio::input;

fn main() {
    input! {
        r:f64,
        x:f64,
        y:f64,
    };
    let l = (x * x + y * y).sqrt();

    if r > l {
        println!("{}", 2);
        return;
    }
    println!("{}", (l / r).ceil());
}
