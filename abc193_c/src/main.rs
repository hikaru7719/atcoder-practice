use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut result: HashSet<usize> = HashSet::new();
    for a in 2..(n + 1) {
        let mut num = a * a;
        if num > n {
            break;
        }
        for _b in 2..(n + 1) {
            if num <= n {
                result.insert(num);
            } else {
                break;
            }
            num *= a;
        }
    }
    println!("{}", n - result.len());
}
