use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n:i32,
        v: [i32; n],
    };
    let mut set: HashSet<i32> = HashSet::new();
    for i in v.iter() {
        set.insert(*i);
    }
    println!("{}", set.len());
}
