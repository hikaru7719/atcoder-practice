use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64;n],
    };
    let mut b = a.clone();
    let mut c: Vec<u64> = vec![0; n];
    b.sort();
    for i in 0..k {
        // if a.len() - 1 < i + k {
        //     break;
        // }
        let mut count = 0;
        let mut vv: Vec<u64> = Vec::new();
        loop {
            if a.len() - 1 < i + count {
                break;
            }
            vv.push(a[i + count]);
            count += k;
        }
        vv.sort();

        let mut count2 = 0;
        for v in vv.iter() {
            c[i + count2] = *v;
            count2 += k;
        }
    }

    // println!("{:?}", c);
    // println!("{:?}", b);
    if c == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
