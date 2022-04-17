use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        a: [i128; n],
        q: usize,
        ql: [(usize, usize, i128); q],
    };
    let mut map: HashMap<i128, Vec<usize>> = HashMap::new();
    for (u, aa) in a.iter().enumerate() {
        if let Some(x) = map.get(aa) {
            let mut xx = x.clone();
            xx.push(u + 1);
            map.insert(*aa, xx.clone());
        } else {
            map.insert(*aa, vec![u + 1]);
        }
    }

    // println!("{:?}", map);
    for (l, r, x) in ql.iter() {
        if let Some(xx) = map.get(x) {
            let li = xx.binary_search(&l).map_or_else(|e| e, |v| v);
            let ri = xx.binary_search(&(r + 1)).map_or_else(|e| e, |v| v);
            println!("{}", ri - li)
        } else {
            println!("{}", 0);
        }
    }
}
