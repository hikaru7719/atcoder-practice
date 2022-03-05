use proconio::input;

fn main() {
    input! {
        n:i32,
        s: [[str;n+1];n+1],
    }
    println!("{:?}", s);
}
