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

    let mut min_j = 0;
    let mut count = 0;
    for (i2, row) in v2.iter().enumerate() {
        let (b, local_min_j) = row_include(min_j, i2, row, &v);
        if b == true {
            min_j = local_min_j;
            count += 1;
        }
    }
    if h2 <= count {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn row_include(min_j: usize, i2: usize, row2: &Vec<i64>, vec: &Vec<Vec<i64>>) -> (bool, usize) {
    for (i, row) in vec.iter().enumerate() {
        let mut row_r = 0;
        let mut min_i = 0;
        let mut local_min_j = 0;
        for (j, val) in row.iter().enumerate() {
            local_min_j = j;
            if min_j != 0 && j < min_j {
                continue;
            }
            for (j2, val2) in row2.iter().enumerate() {
                if (min_i == 0 || min_i < i) && i2 <= i && j2 <= j && val2 == val {
                    row_r += 1;
                    min_i = i;
                }
            }
        }
        if row2.len() <= row_r {
            return (true, local_min_j);
        }
    }
    return (false, 0);
}
