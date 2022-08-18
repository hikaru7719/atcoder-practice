use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    };
    let mut map: HashMap<String, i32> = HashMap::new();

    for ss in s.iter() {
        if let Some(x) = map.get(ss) {
            println!("{}({})", ss, x);
            let sss = ss.to_string();
            let some = x + 1;
            map.insert(sss, some);
        } else {
            println!("{}", ss);
            map.insert(ss.to_string(), 1);
        }
    }
}
