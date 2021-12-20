use std::mem::swap;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();

    if s == t {
        println!("Yes");
        return;
    }

    for i in 0..n - 1 {
        let mut copy = s.clone();

        copy.swap(i, i + 1);

        if copy == t {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
