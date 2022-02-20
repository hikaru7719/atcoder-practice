use proconio::input;

fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        d:i32,
    };
    let z = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199,
    ];
    let mut result: Vec<bool> = vec![];
    for x in a..b + 1 {
        let mut flag = false;
        for y in c..d + 1 {
            let ans = x + y;
            for s in z.iter() {
                if ans == *s {
                    flag = true;
                }
            }
        }
        result.push(flag);
    }

    if result.iter().any(|e| *e == false) {
        println!("{}", "Takahashi");
    } else {
        println!("{}", "Aoki")
    }
}
