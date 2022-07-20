use proconio::input;

fn main() {
    input! {
        mut n: String,
        k: usize,
    };

    for _ in 0..k {
        n = calc(n.clone());
    }
    println!("{}", n);
}

fn calc(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();

    let mut num = 0;
    for (i, c) in chars.iter().enumerate() {
        num += 8_f64.powi(i as i32) as i128 * c.to_digit(10).unwrap() as i128;
    }

    let mut v: Vec<i128> = Vec::new();
    loop {
        if num == 0 {
            break;
        }
        v.push(num % 9);
        num /= 9;
    }
    if v.is_empty() {
        v.push(0);
    }
    v.reverse();
    v.into_iter()
        .map(|x| {
            if x == 8 {
                "5".to_string()
            } else {
                x.to_string()
            }
        })
        .collect()
}
