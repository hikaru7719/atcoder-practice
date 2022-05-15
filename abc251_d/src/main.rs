use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        w: i128,
        mut a: [i128;n],
    }
    a.push(0);
    let mut map: HashMap<i128, bool> = HashMap::new();
    for aa in a.iter() {
        if *aa < w + 1 {
            map.insert(*aa, true);
        }
    }

    for (i, aa) in a.iter().enumerate() {
        for (j, bb) in a.iter().enumerate() {
            if i == j {
                continue;
            }
            let r = aa + bb;
            if r < w + 1 {
                map.insert(r, true);
            }
        }
    }
    for (i, aa) in a.iter().enumerate() {
        for (j, bb) in a.iter().enumerate() {
            for (k, cc) in a.iter().enumerate() {
                if i == j {
                    continue;
                }
                if i == k {
                    continue;
                }
                if j == k {
                    continue;
                }
                let r = aa + bb + cc;
                if r < w + 1 {
                    map.insert(r, true);
                }
            }
        }
    }

    println!("{}", map.len());
}
