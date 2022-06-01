use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        q: usize,
    }
    let mut map: BTreeMap<u64, u64> = BTreeMap::new();
    for _ in 0..q {
        input! {query: usize};

        match query {
            1 => {
                input! {x: u64};
                if let Some(count) = map.get(&x) {
                    let c = count + 1;
                    map.insert(x, c);
                } else {
                    map.insert(x, 1);
                }
            }
            2 => {
                input! {x: u64, c: u64};
                if let Some(count) = map.get(&x) {
                    let num = *count - std::cmp::min(c, *count);
                    if num < 1 {
                        map.remove(&x);
                    } else {
                        map.insert(x, num);
                    }
                }
            }
            3 => {
                let min = map.iter().next().unwrap();
                let max = map.iter().last().unwrap();
                println!("{}", max.0 - min.0);
            }
            _ => {
                panic!("error")
            }
        }
        // println!("{:?}", &map);
    }
}
