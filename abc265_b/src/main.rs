use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        a: [i64;n-1],
        xy: [(i64,i64);m],
    };

    let mut map: HashMap<i64, i64> = HashMap::new();

    for (x, y) in xy.into_iter() {
        map.insert(x, y);
    }

    for (i, aa) in a.iter().enumerate() {
        t -= aa;

        if t <= 0 {
            println!("No");
            return;
        }
        if let Some(x) = map.get(&((i + 2) as i64)) {
            t += x;
        }
    }
    println!("Yes");
}
