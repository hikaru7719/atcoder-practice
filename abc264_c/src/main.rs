use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        v: [[i64;w]; h],
    };

    input! {
        h2: usize,
        w2: usize,
        v2: [[i64;w2]; h2],
    };

    for hh in 0..=1 << h {
        for ww in 0..=1 << w {
            let hvec: Vec<usize> = (0..h).into_iter().filter(|i| hh & 1 << i != 0).collect();
            let wvec: Vec<usize> = (0..w).into_iter().filter(|j| ww & 1 << j != 0).collect();

            if hvec.len() != h2 || wvec.len() != w2 {
                continue;
            }

            let mut result = true;
            for hh2 in 0..h2 {
                for ww2 in 0..w2 {
                    if v[hvec[hh2]][wvec[ww2]] != v2[hh2][ww2] {
                        result = false
                    }
                }
            }

            if result {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
