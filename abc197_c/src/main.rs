use proconio::input;

fn main() {
    input! {
        n: i32,
        vec: [i32; n],
    };

    let mut res = std::i32::MAX;
    for i in 0..1 << (n - 1) {
        let mut xored = 0;
        let mut ored = 0;
        for j in 0..=n {
            if j < n {
                ored = ored | vec[j as usize];
            }
            if j == n || (i >> j) & 1 == 1 {
                xored = xored ^ ored;
                ored = 0;
            }
        }
        if xored < res {
            res = xored;
        }
    }
    println!("{}", res);
}
