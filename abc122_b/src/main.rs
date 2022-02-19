use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut v: Vec<String> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    while left < s.len() {
        for (n, _) in chars.iter().enumerate() {
            if n < left {
                continue;
            }
            match s.get(left..n + 1) {
                Some(result) => v.push(result.to_string()),
                None => continue,
            }
        }
        left += 1;
    }
    v.push("".to_string());
    let mut maxlen = 0;
    for s in v {
        if contains(&s) {
            continue;
        } else {
            if maxlen < s.len() {
                // println!("{}", s);
                maxlen = s.len();
            }
        }
    }

    println!("{}", maxlen);
}

fn contains(s: &String) -> bool {
    if s.contains("B")
        || s.contains("D")
        || s.contains("E")
        || s.contains("F")
        || s.contains("H")
        || s.contains("I")
        || s.contains("J")
        || s.contains("K")
        || s.contains("L")
        || s.contains("M")
        || s.contains("N")
        || s.contains("O")
        || s.contains("P")
        || s.contains("Q")
        || s.contains("R")
        || s.contains("S")
        || s.contains("U")
        || s.contains("V")
        || s.contains("W")
        || s.contains("X")
        || s.contains("Y")
        || s.contains("Z")
    {
        return true;
    }
    false
}
