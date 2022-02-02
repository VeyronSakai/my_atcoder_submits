use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut a: Vec<Vec<i64>> = Vec::new();

    for _ in 0..h {
        input! {
            tmp_a: [i64; w]
        }

        a.push(tmp_a);
    }

    for i in 0..w {
        for j in 0..h {
            print!("{} ", a[j][i]);
        }

        println!("");
    }
}
