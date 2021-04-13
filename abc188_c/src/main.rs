use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u32,
        a: [i64;2_i64.pow(n)],
    }
    let mut list: Vec<usize>;
    let mut list2: Vec<usize> = std::vec::Vec::<usize>::new();
    for i in 1..=2_i64.pow(n) as usize {
        list2.push(i);
    }
    loop {
        if n == 1 {
            if a[list2[1] - 1_usize] > a[list2[0] - 1_usize] {
                println!("{}", list2[0]);
            } else {
                println!("{}", list2[1]);
            }
            break;
        }
        list = std::vec::Vec::<usize>::new();
        for i in 1..=2_i64.pow(n - 1) as usize {
            let x = list2[(2 * i - 2)];
            let y = list2[(2 * i - 1)];
            if a[x - 1] > a[y - 1] {
                list.push(x);
            } else {
                list.push(y);
            }
        }
        n = n - 1;
        list2 = list;
    }
}
