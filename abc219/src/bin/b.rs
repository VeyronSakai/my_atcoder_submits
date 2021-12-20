use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t : Chars,
    }

    for &v in &t {
        let tmp = v.to_digit(10).unwrap();
        match tmp {
            1 => print!("{}", s1),
            2 => print!("{}", s2),
            3 => print!("{}", s3),
            _ => unreachable!()
        }
    }
}
