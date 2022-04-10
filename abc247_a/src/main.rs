use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ss: Vec<char> = s.chars().collect();
    let mut result = vec!['0'];
    for sss in ss.iter() {
        result.push(*sss);
    }
    result.remove(result.len() - 1);
    let r: String = result.iter().collect();
    println!("{}", r);
}
