use std::io::{stdin, stdout, BufReader, Write};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: i32,
    };
    let mut v: Vec<i32> = Vec::new();
    for i in 0..2 * n + 1 {
        v.push(i + 1);
    }

    println!("{}", v[0]);
    v.remove(0);

    for _ in 0..n + 1 {
        input! {
            from &mut source,
            n: i32
        }
        if n == 0 {
            return;
        }

        if let Some(index) = v.iter().position(|x| *x == n) {
            v.remove(index);
        }
        println!("{}", v[0]);
        stdout().flush().unwrap();
        v.remove(0);
    }
}
