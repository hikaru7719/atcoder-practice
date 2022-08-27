use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ch = s.chars().collect::<Vec<char>>();
    println!("{}", ch[(ch.len() + 1) / 2 - 1]);
}
