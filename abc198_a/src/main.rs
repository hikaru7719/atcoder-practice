use proconio::input;

fn main() {
    input! {
        n: i32
    };
    let mut count = 0;
    for i in 1..n {
        if n - i >= 1 {
            count += 1;
        }
    }
    println!("{}", count);
}
