use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    if *s.last().unwrap() != 's' {
        s.push('s');
    } else {
        s.push('e');
        s.push('s');
    }

    println!("{}", s.iter().collect::<String>());
}
