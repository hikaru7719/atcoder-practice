use proconio::input;

fn main() {
    input! {
        x: i128,
        a: i128,
        d: i128,
        n: i128,
    };

    let xx: i128 = x - a;
    if 0 <= d && 0 <= xx {
        let max = (n - 1) * d;
        if max < xx {
            println!("{}", xx - max);
        } else if d < xx && xx <= max {
            println!("{}", std::cmp::min(xx % d, d - (xx % d)));
        } else {
            println!("{}", std::cmp::min(d - xx, xx));
        }
    } else if d <= 0 && 0 <= xx {
        println!("{}", xx);
    } else if d <= 0 && xx <= 0 {
        let max = (n - 1) * d;
        if xx < max {
            println!("{}", max - xx);
        } else if max <= xx && xx < d {
            println!("{}", std::cmp::min(-xx % -d, -d - (-xx % -d)));
        } else {
            println!("{}", std::cmp::min(xx - d, -xx))
        }
    } else if 0 <= d && xx <= 0 {
        println!("{}", -xx);
    }
}
