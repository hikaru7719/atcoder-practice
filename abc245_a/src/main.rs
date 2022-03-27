use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    if a < c {
        println!("Takahashi");
        return;
    } else if a == c {
        if b < d {
            println!("Takahashi");
            return;
        } else if b == d {
            println!("Takahashi");
            return;
        } else {
            println!("Aoki");
            return;
        }
    } else {
        println!("Aoki");
        return;
    }
}
