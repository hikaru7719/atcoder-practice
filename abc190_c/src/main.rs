use proconio::input;

fn main() {
    input! {
        n:i32,
        m:i32,
        ab: [(i32 , i32); m],
        k: i32,
        cd: [(i32, i32);k]
    };
    println!("{},{},{:?},{},{:?}", n, m, ab, k, cd);
    let mut target = vec![false; (n + 1) as usize];
    let mut max = 0;
    select(&mut target, &cd, 0, &ab, &mut max);
    println!("{}", max);
}

fn count(v: &std::vec::Vec<bool>, ab: &std::vec::Vec<(i32, i32)>, max: &mut i32) {
    let mut counter: i32 = 0;
    for (a, b) in ab.iter() {
        if *v.get(*a as usize).unwrap() && *v.get(*b as usize).unwrap() {
            counter += 1;
        }
    }
    if counter > *max {
        *max = counter
    }
}

fn select(
    target: &mut std::vec::Vec<bool>,
    cd: &std::vec::Vec<(i32, i32)>,
    num: usize,
    ab: &std::vec::Vec<(i32, i32)>,
    max: &mut i32,
) {
    if num as usize == cd.len() {
        println!("{:?}", target);
        count(&target, &ab, &mut *max);
        return;
    }
    let (c, d) = cd[num];
    let mut c_vec = target.clone();
    c_vec[c as usize] = true;
    select(&mut c_vec, &cd, num + 1, ab, max);

    let mut d_vec = target.clone();
    d_vec[d as usize] = true;
    select(&mut d_vec, &cd, num + 1, ab, max);
}
