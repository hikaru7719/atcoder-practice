use proconio::input;

fn main() {
    input! {
        n:i64
    }

    let mux_digit = count_digit(n, 0);
    let mut count = 0;
    for x in 1..=mux_digit {
        if x == mux_digit {
            count += (n - 1000_i64.pow(mux_digit as u32) + 1) * mux_digit;
        } else {
            count += ((1000_i64.pow((x + 1) as u32) - 1) - 1000_i64.pow(x as u32) + 1) * x;
        }
    }
    println!("{}", count);
}

fn count_digit(n: i64, digit: i64) -> i64 {
    if n == 0 {
        return digit - 1;
    }
    let result = n / 1000_i64;
    let new_digit = digit + 1_i64;
    return count_digit(result, new_digit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digit() {
        let result = count_digit(1000, 0);
        assert_eq!(result, 1);
        let result2 = count_digit(1000000, 0);
        assert_eq!(result2, 2);
    }
}
