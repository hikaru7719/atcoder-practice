use proconio::input;

fn main() {
    input! {
        n: String,
    };
    let a = n.chars().collect::<Vec<char>>();
    println!("{}{}", a[a.len() - 2], a[a.len() - 1]);
}
