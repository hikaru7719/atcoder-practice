use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n:i32,
        m:i32,
        a:[i32;n],
        b:[i32;m],
    }
    let mut m: HashMap<i32, i32> = HashMap::new();
    for x in b.iter() {
        match m.get(x) {
            Some(xx) => {
                let yy = *xx + 1;
                let _ = m.insert(*x, yy);
            }
            None => {
                m.insert(*x, 1);
            }
        }
    }

    for (x, y) in m.iter() {
        let c: Vec<i32> = a.iter().filter(|z| **z == *x).map(|x| *x).collect();
        if c.len() < *y as usize {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
