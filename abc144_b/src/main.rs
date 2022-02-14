use proconio::input;

fn main() {
    input! {
        n:i32
    }

    for a in 1..10 {
        for b in 1..10 {
            if a * b == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
