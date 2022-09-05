use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let mut c: Vec<u32> = vec![0; 11];
    for (i, cc) in s.chars().enumerate() {
        c[i + 1] = cc.to_digit(10).unwrap();
    }

    let mut c2: Vec<u32> = vec![0; 10];
    if c[1] == 1 {
        println!("No");
        return;
    } else {
        c2[0] = c[7];
        c2[1] = c[4];
        c2[2] = c[8] + c[2];
        c2[3] = c[5];
        c2[4] = c[3] + c[9];
        c2[5] = c[6];
        c2[6] = c[10];
    }

    for (i, x) in c2.iter().enumerate() {
        for (j, y) in c2.iter().enumerate() {
            if i == j {
                continue;
            }

            if i == 0 && j == 1 {
                continue;
            }

            if i == 1 && (j == 0 || j == 2) {
                continue;
            }

            if i == 2 && (j == 1 || j == 3) {
                continue;
            }

            if i == 3 && (j == 2 || j == 4) {
                continue;
            }

            if i == 4 && (j == 3 || j == 5) {
                continue;
            }

            if i == 5 && (j == 4 || j == 6) {
                continue;
            }

            if i == 6 && j == 5 {
                continue;
            }

            if 0 < *x && 0 < *y {
                if i == 0 {
                    if 0 < c2[i + 1] {
                        continue;
                    }
                } else if i == 6 {
                    if 0 < c2[i - 1] {
                        continue;
                    }
                } else {
                    if i < j && 0 < c2[i + 1] {
                        continue;
                    }
                    if i > j && 0 < c2[i - 1] {
                        continue;
                    }
                }
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
