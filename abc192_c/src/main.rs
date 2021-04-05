use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut result = n;
    for _x in 0..k {
        result = calc(result);
        if result == 0 {
            break;
        }
    }
    println!("{}", result);
}

fn calc(a_i_1: i64) -> i64 {
    let mut v = devide(a_i_1);
    v.sort();
    let mut v2 = v.clone();
    v2.reverse();
    let a = v
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let b = v2
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    return b - a;
}

fn devide(a_i_1: i64) -> std::vec::Vec<i64> {
    let mut devider = 10;
    let mut devider2 = 1;
    let mut v = std::vec::Vec::<i64>::new();
    loop {
        let p = a_i_1 % devider;
        let p2 = p / devider2;
        v.push(p2);
        if p == a_i_1 {
            return v;
        }
        devider = devider * 10;
        devider2 = devider2 * 10;
    }
}
