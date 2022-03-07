use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    let ss: String = chars.into_iter().collect();
    println!("{}", ss);
}
