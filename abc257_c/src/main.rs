use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
        w: [i128; n],
    };
    let ss = s.chars().collect::<Vec<char>>();

    let mut order: Vec<(i128, i32)> = Vec::new();
    let mut ans = 0;
    for i in 0..n {
        match ss[i] {
            '1' => {
                order.push((w[i], 1));
                ans += 1;
            }
            '0' => order.push((w[i], 0)),
            _ => panic!("error"),
        }
    }
    order.sort_by(|(a1, a2), (b1, b2)| {
        if let Ordering::Equal = a1.cmp(b1) {
            return a2.cmp(b2);
        } else {
            return a1.cmp(b1);
        }
    });

    let mut x = ans;
    for i in 0..n {
        if order[i].1 == 1 {
            x -= 1;
        } else {
            x += 1;
        }
        if i + 1 < n {
            if order[i].0 == order[i + 1].0 {
                continue;
            } else {
                ans = std::cmp::max(ans, x);
            }
        } else {
            ans = std::cmp::max(ans, x);
        }
    }
    println!("{}", ans);
}
