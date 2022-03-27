use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut v: Vec<(bool, bool, bool, bool)> = Vec::new();
    for nn in 0..n - 1 {
        let mut r1 = false;
        let mut r2 = false;
        let mut r3 = false;
        let mut r4 = false;
        if (a[nn] - a[nn + 1]).abs() <= k {
            r1 = true;
        }
        if (a[nn] - b[nn + 1]).abs() <= k {
            r2 = true;
        }
        if (b[nn] - a[nn + 1]).abs() <= k {
            r3 = true;
        }
        if (b[nn] - b[nn + 1]).abs() <= k {
            r4 = true;
        }
        v.push((r1, r2, r3, r4));
    }

    let mut currenta = true;
    let mut currentb = true;
    for vv in v.iter() {
        let (r1, r2, r3, r4) = *vv;
        if currenta && !(r1 || r2) && currentb && !(r3 || r4) {
            println!("No");
            return;
        }
        currenta = false;
        currentb = false;
        if r1 {
            currenta = true;
        }
        if r2 {
            currentb = true;
        }
        if r3 {
            currenta = true;
        }
        if r4 {
            currentb = true;
        }
    }
    println!("Yes")
}
