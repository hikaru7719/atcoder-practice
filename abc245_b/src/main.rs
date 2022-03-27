use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };
    let mut r = vec![0; 20001];
    for aa in a.iter() {
        r[*aa as usize] = 1;
    }

    for (x, rr) in r.iter().enumerate() {
        if *rr == 0 {
            println!("{}", x);
            return;
        }
    }
}
