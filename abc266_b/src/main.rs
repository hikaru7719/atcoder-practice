use proconio::input;

fn main() {
    input! {
        mut n: i128,
    };

    if n % 998244353 == 0 {
        println!("{}", 0);
    } else if n < 0 {
        n = -n;
        let div = n / 998244353_i128;
        println!("{}", 998244353_i128 * (div + 1) - n);
    } else {
        println!("{}", n % 998244353);
    }
}
