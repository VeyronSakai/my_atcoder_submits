use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if (i + 1) % 2 == 0 && !s[i].is_uppercase() {
            println!("No");
            return;
        }

        if (i + 1) % 2 == 1 && !s[i].is_lowercase() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
