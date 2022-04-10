use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let s = calc(n, 1, "".to_string());
    println!("{}", s);
}

fn calc(n: usize, i: usize, base: String) -> String {
    if n < i {
        return base;
    }
    if i == 1 {
        return calc(n, i + 1, "1".to_string());
    }

    return calc(n, i + 1, format!("{} {} {}", base, i, base));
}
