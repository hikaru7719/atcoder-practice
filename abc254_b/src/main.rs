use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut result: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        let mut list: Vec<i32> = Vec::new();
        for j in 0..i + 1 {
            if j == 0 {
                list.push(1);
            } else if i == j {
                list.push(1);
            } else {
                list.push(result[i - 1][j - 1] + result[i - 1][j]);
            }
        }
        result.push(list);
    }
    for r1 in result.iter() {
        for (i, r2) in r1.iter().enumerate() {
            if i == 0 {
                print!("{}", r2)
            } else {
                print!(" {}", r2);
            }
        }
        print!("\n");
    }
}
