use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64,
        p: [(u64, u64);n],
        s: String,
    }
    let ss: Vec<char> = s.chars().collect();

    let mut map: HashMap<(u64, char), (u64, u64)> = HashMap::new();
    for (i, (x, y)) in p.iter().enumerate() {
        if let Some((min, max)) = map.get(&(*y, ss[i])) {
            let t = (std::cmp::min(*min, *x), std::cmp::max(*max, *x));
            map.insert((*y, ss[i]), t);
        } else {
            map.insert((*y, ss[i]), (*x, *x));
        }
    }
    for (i, (x, y)) in p.iter().enumerate() {
        if ss[i] == 'L' {
            if let Some((min, _)) = map.get(&(*y, 'R')) {
                if min < x {
                    println!("{}", "Yes");
                    return;
                }
            }
        } else if ss[i] == 'R' {
            if let Some((_, max)) = map.get(&(*y, 'L')) {
                if x < max {
                    println!("{}", "Yes");
                    return;
                }
            }
        }
    }
    println!("{}", "No");
}
