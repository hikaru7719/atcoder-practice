use proconio::input;

fn main() {
    input! {
        n:i32,
        x:i32,
        v:[(i32, i32); n]
    }
    for i in 0..1 << n {
        let mut sum = 0;
        for j in 1..n + 1 {
            let (x, y) = v[(j - 1) as usize];
            if i & 1_i32 << (j - 1) == 0 {
                sum += x;
            } else {
                sum += y;
            }
        }

        if sum == x {
            println!("Yes");
            return;
        }
    }
    println!("{}", "No");
}
