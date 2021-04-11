use proconio::input;

fn main() {
    input! {
        n: String,
    };
    let mut res = false;
    for i in 0..10 {
        let str = add(0, i, n.clone());
        if reverse_bool(str) {
            res = true;
            break;
        }
    }
    if res == false {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn add(current: i32, n: i32, s: String) -> String {
    if current == n {
        return s;
    }
    let a = "0".to_string() + &s;
    return add(current + 1, n, a);
}

fn reverse_bool(s: String) -> bool {
    let c = s.clone();
    let result = c.chars().rev().collect::<String>();
    if result == s {
        return true;
    }
    return false;
}
