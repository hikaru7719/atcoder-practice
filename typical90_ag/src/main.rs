use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
    };
    if h == 1 || w == 1 {
        println!("{}", h * w);
    } else {
        println!("{}", ((h + 1) / 2) * ((w + 1) / 2));
    }
}
