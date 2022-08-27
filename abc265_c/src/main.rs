use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    for _ in 0..h {
        input! {
            n: String,
        };
        let vc: Vec<char> = n.chars().collect();
        vec.push(vc);
    }

    let mut i = 0;
    let mut j = 0;

    loop {
        if map.get(&(i, j)).is_some() {
            println!("{}", -1);
            return;
        } else {
            map.insert((i, j), true);
        }

        match vec[i][j] {
            'U' => {
                if i == 0 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                i -= 1
            }
            'D' => {
                if i == h - 1 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                i += 1
            }
            'L' => {
                if j == 0 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                j -= 1
            }
            'R' => {
                if j == w - 1 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                j += 1
            }
            _ => panic!("error"),
        }
    }
}
