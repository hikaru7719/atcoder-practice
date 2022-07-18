use proconio::input;

fn main() {
    input! {
        _: i32,
        m: i32,
        x: i32,
        t: i32,
        d: i32,
    };

    if x <= m {
        println!("{}", t);
    } else {
        println!("{}", t - d * (x - m));
    }
}
