use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
        a: i32,
    }

    if n == 1 {
        println!("1");
        return;
    }

    if k + a - 1 <= n {
        println!("{}", k + a - 1);
    } else {
        println!("{}", (k % n - 1 + a) % (n + 1));
    }
}