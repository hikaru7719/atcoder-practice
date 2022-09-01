use proconio::input;

fn main() {
    input! {
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    d: (i32, i32),
    };

    if cross(vec(b, a), vec(d, a)) < 0 {
        println!("No");
        return;
    }

    if cross(vec(c, b), vec(a, b)) < 0 {
        println!("No");
        return;
    }

    if cross(vec(d, c), vec(b, c)) < 0 {
        println!("No");
        return;
    }

    if cross(vec(a, d), vec(c, d)) < 0 {
        println!("No");
        return;
    }

    println!("Yes");
}

fn cross(x: (i32, i32), y: (i32, i32)) -> i32 {
    x.0 * y.1 - x.1 * y.0
}

fn vec(x: (i32, i32), y: (i32, i32)) -> (i32, i32) {
    (x.0 - y.0, x.1 - y.1)
}
