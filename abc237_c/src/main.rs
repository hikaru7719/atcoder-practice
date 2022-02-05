use proconio::input;

fn main() {
    input! {
        mut n: String,
    };
    let mut count = 0;
    for a in n.clone().as_str().chars().rev() {
        if a == 'a' {
            count += 1;
        }
        break;
    }
    for _ in 0..count {
        let reverse = n.clone().as_str().chars().rev().collect::<String>();
        if n == reverse {
            println!("{}", "Yes");
            return;
        }
        n = format!("{}{}", 'a', n);
    }
    let reverse = n.clone().as_str().chars().rev().collect::<String>();
    if n == reverse {
        println!("{}", "Yes");
    }else {
        println!("{}", "No");
    }
}
