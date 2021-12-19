use proconio::input;
use proconio::marker::Chars;

fn main() {
    println!("{}", b41);
    input! {
        s: Chars
    }

    if s[2] == s[3] && s[4] == s[5] {
        println!("Yes");
    } else {
        println!("No");
    }
}
