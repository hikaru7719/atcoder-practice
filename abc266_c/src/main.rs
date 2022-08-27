use proconio::input;

fn main() {
    input! {
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    d: (i32, i32),
    };

    if check(a, c, b, d) {
        println!("No");
        return;
    }
    if check(b, d, a, c) {
        println!("No");
        return;
    }
    println!("Yes");
}

fn check(a: (i32, i32), c: (i32, i32), b: (i32, i32), d: (i32, i32)) -> bool {
    let x1: i32 = std::cmp::max(b.0, d.0);
    let x2: i32 = std::cmp::min(b.0, d.0);
    let y1: i32 = std::cmp::max(b.1, d.1);
    let y2: i32 = std::cmp::min(b.1, d.1);
    if a.0 < x2 && c.0 < x2 {
        return true;
    }
    if x1 < a.0 && x1 < c.0 {
        return true;
    }
    if a.1 < y2 && c.1 < y2 {
        return true;
    }

    if y1 < a.1 && y1 < c.1 {
        return true;
    }
    false
}
