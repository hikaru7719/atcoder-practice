use proconio::input;

fn main() {
    input! {
        n:i32,
        m:i32,
    }

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m {
        input! {
            k:i32,
            v: [i32;k]
        }
        vec.push(v.iter().map(|x| *x).collect());
    }

    input! {
        p: [i32; m]
    }

    let mut ans = 0;
    for i in 0..1 << n {
        let mut v2: Vec<bool> = Vec::new();
        for j in 0..n {
            if i & 1 << j != 0 {
                v2.push(true);
            } else {
                v2.push(false);
            }
        }
        let mut count = 0;
        for (i, vv) in vec.iter().enumerate() {
            if vv
                .iter()
                .map(|x| v2[(x - 1) as usize])
                .filter(|x| *x)
                .collect::<Vec<bool>>()
                .len()
                % 2
                == p[i] as usize
            {
                count += 1;
            }
        }
        if count == m {
            ans += 1;
        }
    }
    println!("{}", ans);
}
