use proconio::input;

fn main() {
    input! {
        s: String
    };
    let mut x = s.chars();
    let a = x.next().unwrap();
    let b = x.next().unwrap();
    let c = x.next().unwrap();
    println!("{}{}{}", b, c, a);
}
