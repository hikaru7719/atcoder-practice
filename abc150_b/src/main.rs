use proconio::input;

fn main() {
    input! {
        _:i32,
        s:String
    }

    let mut count = 0;
    let mut seek = 0;
    for c in s.chars() {
        if c == 'A' {
            seek = 1;
        } else if seek == 1 {
            if c == 'B' {
                seek = 2;
            } else {
                seek = 0;
            }
        } else if seek == 2 {
            if c == 'C' {
                count += 1;
                seek = 0;
            } else {
                seek = 0;
            }
        }
    }
    println!("{}", count);
}
