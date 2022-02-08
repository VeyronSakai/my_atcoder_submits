use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
        a: i32,
    }

    let tmp = ((a - 1) + k) % n;
    if tmp == 0 {
        println!("{}", n);
    } else {
        println!("{}", tmp);
    }
}