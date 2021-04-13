use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        s: [String; n]
    };
    let l: HashSet<String> = s.into_iter().collect();
    let clone = &l.clone();
    let mut result = false;
    let mut ch = String::new();
    for x in l.iter() {
        if x.contains("!") {
            let y = x.get(1..).unwrap();
            if clone.contains(y) {
                result = true;
                ch = y.to_string();
                break;
            }
        } else {
            let y = "!".to_string() + &x.clone().to_string();
            if clone.contains(y.as_str()) {
                result = true;
                ch = x.to_string();
                break;
            }
        }
    }
    if result {
        println!("{}", ch);
    } else {
        println!("satisfiable")
    }
}
