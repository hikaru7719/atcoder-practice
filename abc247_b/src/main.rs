use proconio::input;
fn main() {
    input! {
        n: usize,
        v: [(String,String);n],
    };

    let v2 = v.clone();
    for (i, (x, y)) in v.iter().enumerate() {
        let mut count = 0;
        for (j, (x2, y2)) in v2.iter().enumerate() {
            if i != j && *x == *x2 {
                count += 1;
            }
            if i != j && *x == *y2 {
                count += 1;
            }
            if i != j && *y == *y2 {
                count += 1;
            }
            if i != j && *y == *x2 {
                count += 1;
            }
        }
        // println!("{}", count);
        if 1 < count {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
