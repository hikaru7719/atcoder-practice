use proconio::input;
fn main() {
    input! {
        n: usize,
        v: [(i64, i64, i64); n],
    }

    let mut max: i64 = std::i64::MAX;
    for (a, b, c) in v.iter() {
        if c - a > 0 {
            if max > *b {
                max = *b;
            }
        }
    }
    if max == std::i64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", max);
    }
}
