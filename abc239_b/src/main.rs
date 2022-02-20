use proconio::input;

fn main() {
    input! {
        x:i64
    };
    if 0 < x {
        println!("{}", x / 10);
    } else {
        let b = x.rem_euclid(10);
        if b != 0 {
            println!("{}", x / 10 - 1);
        } else {
            println!("{}", x / 10);
        }
    }
    // let a = x.div_euclid(10_f64);
    // let b = x.rem_euclid(10_f64);
}
