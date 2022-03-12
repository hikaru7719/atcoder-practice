use proconio::input;

fn main() {
    input! {
        v: i32,
        a:i32,
        b:i32,
        c:i32,
    };
    let mut z = v;
    loop {
        if z < a {
            println!("{}", "F");
            return;
        }
        z -= a;
        if z < b {
            println!("{}", "M");
            return;
        }
        z -= b;
        if z < c {
            println!("{}", "T");
            return;
        }
        z -= c;
    }
}
