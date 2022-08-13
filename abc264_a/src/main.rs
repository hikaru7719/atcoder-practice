use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    let vec: Vec<char> = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let result = vec.get(l - 1..r).unwrap();
    println!("{}", result.iter().collect::<String>());
}
