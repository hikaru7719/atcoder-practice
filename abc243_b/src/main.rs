use proconio::input;
fn main() {
    input! {
        n:usize,
        a: [i32;n],
        b: [i32;n],
    }
    let mut count1 = 0;
    for i in 0..n {
        if a[i] == b[i] {
            count1 += 1;
        }
    }
    println!("{}", count1);

    let mut count2 = 0;
    for (i, aa) in a.iter().enumerate() {
        for (j, bb) in b.iter().enumerate() {
            if *aa == *bb && i != j {
                count2 += 1;
            }
        }
    }

    println!("{}", count2);
}
