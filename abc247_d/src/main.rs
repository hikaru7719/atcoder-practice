use proconio::fastout;
use std::io;

#[fastout]
fn main() {
    let mut q = String::new();
    io::stdin().read_line(&mut q).unwrap();
    let qq = q.trim_end().parse::<usize>().unwrap();
    let mut v: Vec<(usize, usize)> = Vec::new();
    for _ in 0..qq {
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let a = query.trim().split(" ").collect::<Vec<&str>>();
        if a.len() == 3 {
            let x = a[1].parse::<usize>().unwrap();
            let c = a[2].parse::<usize>().unwrap();
            v.push((x, c))
        } else {
            let c = a[1].parse::<usize>().unwrap();
            println!("{}", get_result(&mut v, c));
        }
    }
}

fn get_result(v: &mut Vec<(usize, usize)>, target: usize) -> usize {
    let mut tt = target;
    let mut result: usize = 0;
    loop {
        let (x, c) = v[0];
        if c < tt {
            tt = tt - c;
            result += x * c;
            v.remove(0);
        } else {
            v[0] = (x, c - tt);
            result += x * tt;
            return result;
        }
    }
}
