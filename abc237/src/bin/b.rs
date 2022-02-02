use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h]
    }

    for i in 0..w {
        for j in 0..h {
            print!("{} ", a[j][i]);
        }

        println!("");
    }
}
