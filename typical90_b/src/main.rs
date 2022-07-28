use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut result: Vec<String> = Vec::new();
    for i in 0..2_i32.pow(n as u32) as usize {
        let mut v: Vec<char> = Vec::new();
        let mut count = 0;
        for j in (0..n).rev() {
            if count < 0 {
                break;
            }
            if i & 1 << j != 0 {
                v.push('(');
                count += 1;
            } else {
                v.push(')');
                count -= 1;
            }
        }
        if count != 0 {
            continue;
        } else {
            result.push(v.iter().collect::<String>())
        }
    }
    result.sort();
    for s in result.into_iter() {
        println!("{}", s);
    }
}
