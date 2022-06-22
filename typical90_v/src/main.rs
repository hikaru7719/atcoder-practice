use proconio::input;

fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128,
    };
    let g1 = gcd(a, b);
    let g2 = gcd(c, g1);

    println!("{}", a / g2 + b / g2 + c / g2 - 3);
}

fn gcd(x: i128, y: i128) -> i128 {
    if y == 0 {
        return x;
    } else {
        return gcd(y, x % y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 9), 3);
        assert_eq!(gcd(390, 273), 39);
    }
}
