use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let ss: Vec<char> = s.chars().collect();
    let tt: Vec<char> = t.chars().collect();
    let mut result = true;

    let mut s_result: Vec<(char, i32)> = Vec::new();
    let mut t_result: Vec<(char, i32)> = Vec::new();

    for (i, s) in ss.iter().enumerate() {
        if i == 0 {
            s_result.push((*s, 1));
        } else {
            let prev = s_result.pop().unwrap();
            if *s == prev.0 {
                s_result.push((prev.0, prev.1 + 1))
            } else {
                s_result.push(prev);
                s_result.push((*s, 1));
            }
        }
    }

    for (i, t) in tt.iter().enumerate() {
        if i == 0 {
            t_result.push((*t, 1));
        } else {
            let prev = t_result.pop().unwrap();
            if *t == prev.0 {
                t_result.push((prev.0, prev.1 + 1))
            } else {
                t_result.push(prev);
                t_result.push((*t, 1));
            }
        }
    }

    if s_result.len() != t_result.len() {
        result = false;
    } else {
        for i in 0..s_result.len() {
            let s = s_result[i];
            let t = t_result[i];
            if s.0 != t.0 {
                result = false;
            } else if s.1 == 1 && 1 < t.1 {
                result = false
            } else if t.1 < s.1 {
                result = false;
            }
        }
    }

    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
