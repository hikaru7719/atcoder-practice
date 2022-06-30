use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut list: Vec<i64> = Vec::new();
    for _ in 0..q {
        input! {
            t: i32,
            x: i64,
        };

        match t {
            1 => list.insert(0, x),
            2 => list.push(x),
            3 => println!("{}", list[(x - 1) as usize]),
            _ => panic!("error"),
        };
    }
}
