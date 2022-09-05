use proconio::input;

fn main() {
    input! {
        s: String,
    };
    if s == "Monday" {
        println!("{}", 5);
    }

    if s == "Tuesday" {
        println!("{}", 4);
    }

    if s == "Wednesday" {
        println!("{}", 3);
    }

    if s == "Thursday" {
        println!("{}", 2);
    }

    if s == "Friday" {
        println!("{}", 1);
    }
}
