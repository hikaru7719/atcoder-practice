use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        poj: [(String, i128); n],
    }
    let mut map: HashMap<String, bool> = HashMap::new();
    let mut v: Vec<(i128, usize)> = vec![];

    for (i, (s, t)) in poj.iter().enumerate() {
        if map.get(&s.clone()).is_some() {
            continue;
        }
        map.insert(s.clone(), true);
        v.push((*t, i + 1));
    }

    // println!("{:?}", v);
    v.sort_by(|(a1, a2), (b1, b2)| {
        if a1 < b1 {
            return std::cmp::Ordering::Greater;
        } else if a1 == b1 {
            if a2 > b2 {
                return std::cmp::Ordering::Greater;
            } else if a2 < b2 {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Equal;
            }
        } else {
            return std::cmp::Ordering::Less;
        }
    });
    // println!("{:?}", v);
    println!("{}", v[0].1);
}
