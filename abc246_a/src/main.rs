use proconio::input;

fn main() {
    input! {
        a: (i32, i32),
        b: (i32, i32),
        c: (i32, i32),
    }
    let x;
    let y;
    if a.0 == b.0 {
        x = c.0;
    } else if a.0 == c.0 {
        x = b.0;
    } else {
        x = a.0;
    }

    if a.1 == b.1 {
        y = c.1;
    } else if a.1 == c.1 {
        y = b.1;
    } else {
        y = a.1;
    }
    println!("{} {}", x, y);
}
