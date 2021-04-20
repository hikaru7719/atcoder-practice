use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    let mut result = 0;
    for i in 1..=n {
        let a = format!("{}", i);
        let b = format!("{:o}", i);
        if a.contains('7') {
            continue;
        }
        if b.contains('7') {
            continue;
        }
        result += 1;
    }
    println!("{}", result);
}
