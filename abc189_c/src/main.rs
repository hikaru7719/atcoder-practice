use proconio::input;

fn main() {
    input! {
        n:i32,
        a:[i32;n]
    }
    let mut result = 0;
    for x in 0..n {
        let mut m = a[x as usize];
        for y in x..n {
            m = std::cmp::min(m, a[y as usize]);
            result = std::cmp::max(result, m * (y - x + 1));
        }
    }
    println!("{}", result);
}
