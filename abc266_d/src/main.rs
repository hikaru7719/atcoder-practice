use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        t: [(i64, i64,i64);n],
    };
    let mut map: HashMap<i64, (i64, i64)> = HashMap::new();
    for x in t.iter() {
        map.insert(x.0, (x.1, x.2));
    }

    let mut vec = vec![vec![0; 5]; (t[n - 1].0 + 1) as usize];
    vec[0][1] = -1000000000000;
    vec[0][2] = -1000000000000;
    vec[0][3] = -1000000000000;
    vec[0][4] = -1000000000000;

    for i in 1..=t[n - 1].0 as usize {
        for j in 0..5 {
            let sunuke = map.get(&(i as i64)).unwrap_or(&(-1, 0));
            let mut v = 0;
            if sunuke.0 == j as i64 {
                v = sunuke.1;
            }

            if j == 0 {
                vec[i][j] = std::cmp::max(vec[i - 1][j] + v, vec[i - 1][j + 1] + v);
                continue;
            }
            if j == 4 {
                vec[i][j] = std::cmp::max(vec[i - 1][j] + v, vec[i - 1][j - 1] + v);
                continue;
            }

            vec[i][j] = std::cmp::max(
                std::cmp::max(vec[i - 1][j] + v, vec[i - 1][j + 1] + v),
                vec[i - 1][j - 1] + v,
            );
        }
    }

    println!("{}", vec[(vec.len() - 1) as usize].iter().max().unwrap());
}
