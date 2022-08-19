use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    };

    println!("{}", convert(n, true, x, y));
}

fn convert(level: i64, is_red: bool, x: i64, y: i64) -> i64 {
    if level == 1 {
        if is_red {
            return 0;
        } else {
            return 1;
        }
    }
    if is_red {
        convert(level - 1, true, x, y) + convert(level, false, x, y) * x
    } else {
        convert(level - 1, true, x, y) + convert(level - 1, false, x, y) * y
    }
}
