use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if x > y {
        println!("{}", x);
        return;
    }

    if x < y {
        println!("{}", y);
        return;
    }
}
