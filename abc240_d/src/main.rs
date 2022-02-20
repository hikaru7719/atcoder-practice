use proconio::input;

fn main() {
    input! {
        n: i32,
        v: [i32; n]
    };
    let mut vec: Vec<[i32; 2]> = Vec::new();
    let mut last = -1;
    let mut count: usize = 0;
    for vv in v.iter() {
        if last == *vv {
            vec[count - 1][1] += 1;
        } else {
            last = *vv;
            count += 1;
            vec.push([*vv, 1]);
        }
    }
    println!("{:?}", vec);
    let mut sum = 0;
    for v in vec.iter() {
        consume(*v, &mut sum)
    }
}

fn consume(v: [i32; 2], sum: &mut i32) {
    if v[0] > v[1] {
        for _ in 0..v[1] {
            *sum += 1;
            println!("{}", &sum);
        }
    } else {
        let mut counter = 0;
        for _ in 0..v[1] {
            if counter == v[0] {
                *sum -= v[0];
                counter -= v[0];
                println!("{}", &sum);
            }
            *sum += 1;
            counter += 1;
            println!("{}", &sum);
        }
    }
}
