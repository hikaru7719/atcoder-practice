use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        k: f64,
    };
    if b < a {
        println!("{}", 0);
    } else {
        println!("{}", ((b / a).log2() / k.log2()).ceil());
    }
}
