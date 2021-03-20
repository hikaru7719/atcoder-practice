use proconio::input;
fn main() {
    input! {
        a: String,
    };
    if a.contains(".") {
        let b: Vec<&str> = a.split(".").collect();
        println!("{}", b[0]);
    } else {
        println!("{}", a);
    }
}
