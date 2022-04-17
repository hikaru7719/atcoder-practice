use std::result;

use proconio::input;

fn main() {
    // input! {
    //     n: u128,
    //     m: u128,
    //     k: u128,
    // }
    println!("{}", sum(54));
}

fn nn(n: u128, x: u128) -> u128 {
    let mut result = 0;
    for i in 0..n {
        result += x * 10_u128.pow(i as u32);
    }
    return result;
}

fn sum(x: u128) -> u128 {
    let length = x.to_string().len();
    let mut result = 0;
    for i in 0..length {
        result += (x % 10_u128.pow((i + 1) as u32)) / 10_u128.pow((i) as u32);
    }
    return result;
}
