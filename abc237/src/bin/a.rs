use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    if -2_i64.pow(31) <= n && n < 2_i64.pow(31) {
        println!("Yes");
    } else {
        println!("No");
    }
}
