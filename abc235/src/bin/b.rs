use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    for i in 0..n - 1 {
        if h[i + 1] <= h[i] {
            println!("{}", h[i]);
            return;
        }
    }

    println!("{}", h[n - 1]);
}
