use proconio::input;
fn main() {
    input! {
        n: usize,
       mut k: u128,
        x: u128,
       mut a: [u128; n],
    };
    let mut result: Vec<u128> = a
        .iter()
        .map(|a| {
            if k == 0 {
                *a
            } else {
                let r = std::cmp::min(*a / x, k);
                k = k - r;
                *a - r * (x)
            }
        })
        .collect();
    result.sort();
    result.reverse();
    let a = std::cmp::min(result.len(), k as usize);
    result = result[a..].to_vec();
    println!("{}", result.iter().sum::<u128>());
}
