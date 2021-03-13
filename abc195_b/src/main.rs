use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        w: i32
    }

    let target = w * 1000;
    let c = target / a;
    let d = target / b;

    let mut max = 0;
    let mut min = std::i32::MAX;

    for n in d..c + 1 {
        if a * n <= target && target <= b * n {
            if max < n {
                max = n;
            }
            if n < min {
                min = n;
            }
        }
    }
    if max == 0 && min == std::i32::MAX {
        println!("UNSATISFIABLE")
    } else {
        println!("{} {}", min, max);
    }
}
