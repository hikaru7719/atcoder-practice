use proconio::input;

fn main() {
    input! {
        a: i32,
        b:i32
    }
    if (a - b).abs() == 1 {
        println!("Yes");
    } else if a == 1 && b == 10 || a == 10 && b == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
