use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w];h]
    };
    // println!("{:?}", a);
    let mut wlist: Vec<i32> = Vec::new();
    let mut hlist: Vec<i32> = Vec::new();

    for i in 0..h {
        let mut count = 0;
        for j in 0..w {
            count += a[i][j];
        }
        wlist.push(count);
    }

    for j in 0..w {
        let mut count = 0;
        for i in 0..h {
            count += a[i][j];
        }
        hlist.push(count);
    }

    for (i, row) in a.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if j == 0 {
                print!("{}", wlist[i] + hlist[j] - c)
            } else {
                print!(" {}", wlist[i] + hlist[j] - c)
            }
        }
        println!();
    }
}
