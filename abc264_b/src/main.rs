use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    };

    if r == 1 {
        println!("black");
    } else if r == 2 {
        if c == 1 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 3 {
        if c == 1 || (3 <= c && c <= 13) || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 4 {
        if c == 1 || 3 == c || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 5 {
        if c == 1 || 3 == c || (5 <= c && c <= 11) || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 6 {
        if c == 1 || 3 == c || 5 == c || c == 11 || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 7 {
        if c == 1 || 3 == c || 5 == c || (7 <= c && c <= 9) || c == 11 || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 8 {
        //
        if c == 1 || 3 == c || 5 == c || 7 == c || c == 9 || c == 11 || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 9 {
        if c == 1 || 3 == c || 5 == c || (7 <= c && c <= 9) || c == 11 || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 10 {
        if c == 1 || 3 == c || 5 == c || c == 11 || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 11 {
        if c == 1 || 3 == c || (5 <= c && c <= 11) || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 12 {
        if c == 1 || 3 == c || c == 13 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 13 {
        if c == 1 || (3 <= c && c <= 13) || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 14 {
        if c == 1 || c == 15 {
            println!("black");
        } else {
            println!("white");
        }
    } else if r == 15 {
        println!("black");
    }
}
