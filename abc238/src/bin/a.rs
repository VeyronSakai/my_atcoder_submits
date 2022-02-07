use libm::log2;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: f64,
    }

    // 2^n > n^2
    // n > 2log2(n)

    if n > 2.0 * log2(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
