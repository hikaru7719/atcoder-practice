use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a: [i128; n],
        q: usize,
        b: [i128; q],
    };
    a.sort_unstable();

    b.iter().for_each(|bb| {
        let index;
        match a.binary_search(bb) {
            Ok(i) => index = i,
            Err(i) => index = i,
        };
        if 0 < index {
            println!(
                "{}",
                std::cmp::min((a[index] - bb).abs(), (a[index - 1] - bb).abs())
            );
        } else {
            println!("{}", (a[index] - bb).abs())
        }
    });
}
