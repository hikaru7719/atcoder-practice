use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let a: Vec<char> = s.chars().collect();
    let mut v = vec![0; 10];
    // println!("{:?}", v);
    for aa in a.iter() {
        let x = aa.to_digit(10).unwrap();
        v[x as usize] = 1
    }
    for (n, x) in v.iter().enumerate() {
        if *x == 0 {
            println!("{}", n);
        }
    }
}
