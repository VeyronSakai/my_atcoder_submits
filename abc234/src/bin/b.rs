use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", a - b);
}
