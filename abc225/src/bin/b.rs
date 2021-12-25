use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        input: [(i32, i32); n - 1]
    }

    let mut info = vec![0; n];

    for &val in &input {
        info[(val.0 - 1) as usize] += 1;
        info[(val.1 - 1) as usize] += 1;
    }

    for i in 0..n - 1 {
        if info[i] == n - 1 {
            continue;
        }

        if info[i] != 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
