use proconio::input;

fn main() {
    input! {
        n: i128,
        k: usize,
        q: usize,
        mut a: [i128; k],
        l: [usize; q],
    };
    for l in l.into_iter() {
        if a[l - 1] < n {
            if a.len() - 1 == l - 1 {
                a[l - 1] = a[l - 1] + 1;
            } else if a[l - 1] + 1 < a[l] {
                a[l - 1] = a[l - 1] + 1;
            }
        }
    }
    for (n, aa) in a.into_iter().enumerate() {
        if n == 0 {
            print!("{}", aa);
        } else {
            print!(" {}", aa);
        }
    }
    println!("");
}
