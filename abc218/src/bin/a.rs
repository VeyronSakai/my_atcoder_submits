use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        s: Bytes,
    }

    let tmp = s[n - 1] as char;
    if tmp == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
