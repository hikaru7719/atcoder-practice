use proconio::input;

fn main() {
    input! {
        n: usize,
        z: i32,
    };
    let v = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut last: String;
    let mut count = 0;
    loop {
        for ch in v.iter() {
            for _ in 0..n {
                last = ch.to_string();
                count += 1;
                if count == z {
                    println!("{}", last);
                    return;
                }
            }
        }
    }
}
