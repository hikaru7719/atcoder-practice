use proconio::input;
fn main() {
    input! {
        a: i64
    };
    let len = count(a);
    let mut base = 2;
    let mut c = 0;
    loop {
        if len < base {
            break;
        }
        if len == base {
            c += count3(base / 2, devide(a, len));
        } else {
            c += count2(base / 2);
        }
        base += 2;
    }
    println!("{}", c);
}

fn count(a: i64) -> i64 {
    let mut x: i64 = 0;
    loop {
        let res = a / 10_i64.pow(x as u32);
        if res < 1 {
            break;
        }
        x += 1;
    }
    return x;
}

fn count2(a: i64) -> i64 {
    let mut z1 = 0;
    let z2 = 1 * 10_i64.pow((a - 1) as u32);
    for x in 0..a {
        z1 += 9 * 10_i64.pow(x as u32);
    }
    return z1 - z2 + 1;
}

fn count3(a: i64, b: i64) -> i64 {
    let z1 = b;
    let z2 = 1 * 10_i64.pow((a - 1) as u32);
    if z1 < z2 {
        return 0;
    }
    return z1 - z2 + 1;
}

fn devide(a: i64, b: i64) -> i64 {
    let a1 = a % 10_i64.pow((b / 2) as u32);
    let a2 = a / 10_i64.pow((b / 2) as u32);
    if a1 > a2 {
        return a2;
    } else {
        return a1;
    }
}
