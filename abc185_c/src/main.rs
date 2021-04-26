use proconio::input;

fn main() {
    input! {
        l: i128
    };

    let a: i128 = (l - 1_i128)
        * (l - 2_i128)
        * (l - 3_i128)
        * (l - 4_i128)
        * (l - 5_i128)
        * (l - 6_i128)
        * (l - 7_i128)
        * (l - 8_i128)
        * (l - 9_i128)
        * (l - 10_i128)
        * (l - 11_i128);
    let b: i128 = 11 * 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;
    println!("{}", a / b);
}
