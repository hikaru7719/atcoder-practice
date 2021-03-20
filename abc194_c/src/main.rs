use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    let mut v: Vec<Vec<i64>> = vec![vec![0; n as usize]; n as usize];
    let mut count = 0;
    for i in 2..=n {
        for j in 1..=i - 1 {
            if v[(i - 1) as usize][(j - 1) as usize] != 0 {
                count += v[(i - 1) as usize][(j - 1) as usize];
            } else {
                let result = (a[(i - 1) as usize] - a[(j - 1) as usize]).pow(2);
                count += result;
                v[(i - 1) as usize][(j - 1) as usize] = result;
            }
        }
    }
    println!("{}", count);
}
