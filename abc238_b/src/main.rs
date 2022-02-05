use proconio::input;

fn main() {
    input! {
        n: i32,
        v: [i32; n]
    };

    let mut d = 0;
    let mut vec: Vec<i32> = vec![];
    for n in v.iter() {
        d = d + n;
        if d > 360 {
            d = d - 360;
        }
        vec.push(d);
    }
    vec.push(360);
    vec.sort();
    let mut a = 0;
    let mut long = 0; 

    for s in vec.iter() {
        let b = s-a;
        if long < b {
            long = b;
        }
        a = *s;
    }
    println!("{}",long);
}
