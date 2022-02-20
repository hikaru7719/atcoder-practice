use proconio::input;

fn main() {
    input! {
        x1:i128,
        y1:i128,
        x2:i128,
        y2:i128,
    }

    let r1 = calc(x1 - 5, x1 + 5, y1 - 5, y1 + 5, x1, y1);
    let r2 = calc(x2 - 5, x2 + 5, y2 - 5, y2 + 5, x2, y2);
    for (xx1, yy1) in r1.iter() {
        for (xx2, yy2) in r2.iter() {
            if xx1 == xx2 && yy1 == yy2 {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("No");
}

fn calc(a1: i128, a2: i128, b1: i128, b2: i128, xx: i128, yy: i128) -> Vec<(i128, i128)> {
    let mut result: Vec<(i128, i128)> = Vec::new();
    for x in a1..a2 + 1 {
        for y in b1..b2 + 1 {
            if xx.pow(2) - 2 * xx * x + x.pow(2) + yy.pow(2) - 2 * yy * y + y.pow(2) == 5 {
                result.push((x, y));
            }
        }
    }
    result
}
