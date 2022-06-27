use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut map: HashSet<String> = HashSet::new();
    for i in 0..n {
        input! {
            s: String,
        };

        if map.get(&s).is_some() {
            continue;
        }
        map.insert(s);
        println!("{}", i + 1);
    }
}
