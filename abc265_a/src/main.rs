use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
        n: i32,
    };
    let yy = n / 3;
    let xx = n % 3;

    let result = yy * y + xx * x;
    let result2 = n * x;
    if result < result2 {
        println!("{}", result);
    } else {
        println!("{}", result2);
    }
}
