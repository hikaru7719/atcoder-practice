use proconio::input;

fn main() {
    input! {
     s: String
    };
    let ss: Vec<char> = s.chars().collect();
    let mut result: Vec<char> = Vec::new();
    let mut count = 0;
    loop {
        for a in ss.iter() {
            if count == 6 {
                let sss: String = result.iter().collect();
                println!("{}", sss);
                return;
            }
            result.push(*a);
            count += 1;
        }
    }
}
