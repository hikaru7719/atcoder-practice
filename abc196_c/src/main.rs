use proconio::input;
fn main() {
    input! {
        a: i64
    };
    let mut i = 1;
    loop {
        let num: i64 = format!("{}{}", &i.to_string(), &i.to_string())
            .parse()
            .unwrap();
        if a < num {
            println!("{}", i - 1);
            break;
        }
        i += 1;
    }
}
