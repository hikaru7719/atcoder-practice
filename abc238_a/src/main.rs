use proconio::input;

fn main() {
    input!{
        n: f64
    }

    if 2f64.powf(n) > n.powi(2) {
        println!("Yes")
    }else {
        println!("No")
    }
}
