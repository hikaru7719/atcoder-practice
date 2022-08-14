use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let mut chars = s.chars().collect::<Vec<char>>();
    let resutl_chars: Vec<char> = "atcoder".to_string().chars().collect();

    let mut result = 0;
    for (i, c) in resutl_chars.iter().enumerate() {
        let (key, _) = chars.iter().enumerate().find(|(_, b)| **b == *c).unwrap();
        result += swap(i, key, &mut chars);
    }

    println!("{}", result);
}

fn swap(i: usize, key: usize, chars: &mut Vec<char>) -> i32 {
    let mut start = key;
    let mut count = 0;

    while i < start {
        count += 1;

        let tmp = chars[start];
        let tmp2 = chars[start - 1];
        chars[start] = tmp2;
        chars[start - 1] = tmp;

        start -= 1;
    }
    count
}
