use proconio::input;

fn main() {
    input! {
        n:usize,
        k: usize,
        a: [usize;k],
        xy: [(i128, i128); n],
    };
    let mut target: Vec<(i128, i128)> = Vec::new();
    for aa in a.iter() {
        target.push(xy[aa - 1]);
    }

    let mut max = 0_f64;
    for (n, (x2, y2)) in xy.iter().enumerate() {
        if a.contains(&(n + 1)) {
            continue;
        }
        let mut min = 10000000000000_f64;
        for (x1, y1) in target.iter() {
            let len = ((x1 - x2).pow(2) as f64 + (y1 - y2).pow(2) as f64).sqrt();
            if len < min {
                min = len;
            }
        }
        if max < min {
            max = min;
        }
    }
    println!("{}", max);
}
