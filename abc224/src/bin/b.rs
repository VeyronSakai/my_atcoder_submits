use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w] ; h]
    }

    for i2 in 0..h {
        for i1 in 0..i2 {
            for j2 in 0..w {
                for j1 in 0..j2 {
                    if a[i1][j1] + a[i2][j2] > a[i2][j1] + a[i1][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes");
}
