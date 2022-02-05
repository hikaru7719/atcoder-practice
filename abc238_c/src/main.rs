use proconio::input;
fn main() {
    input! {
        n:i128
    }
    let mut sum:i128 = 0;
    for x in 1..n+1 {
        let keta = x.to_string().len() as i32;
        let res = x - (10f64.powi(keta-1) as i128) + 1;
        println!("{}",res);
        sum += res;
    }
    println!("{}", sum % 998244353i128);
}
