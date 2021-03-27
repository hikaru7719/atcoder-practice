use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: i32,
        w: i32,
        x: i32,
        y: i32,
        v: [Chars; h],
    };
    let mut count = 0;
    for xx in 1..h + 1 {
        // (xx, y)が(x, y)から見えるか
        if can_watch(&v, x - 1, y - 1, xx - 1, y - 1) {
            count += 1;
        }
    }

    for yy in 1..w + 1 {
        // (x, yy)が(x,y)から見えるか
        if can_watch(&v, x - 1, y - 1, x - 1, yy - 1) {
            count += 1;
        }
    }
    println!("{}", count - 1);
}

fn can_watch(vec: &std::vec::Vec<std::vec::Vec<char>>, x: i32, y: i32, xx: i32, yy: i32) -> bool {
    let mut true_or_false = true;
    if y == yy {
        if xx < x {
            for a in xx..x + 1 {
                let target = vec[a as usize][y as usize];
                if target == '#' {
                    true_or_false = false;
                }
            }
        } else {
            for a in x..xx + 1 {
                let target = vec[a as usize][y as usize];
                if target == '#' {
                    true_or_false = false;
                }
            }
        }
    }

    if x == xx {
        if yy < y {
            for a in yy..y + 1 {
                let target = vec[x as usize][a as usize];
                if target == '#' {
                    true_or_false = false;
                }
            }
        } else {
            for a in y..yy + 1 {
                let target = vec[x as usize][a as usize];
                if target == '#' {
                    true_or_false = false;
                }
            }
        }
    }
    return true_or_false;
}
