use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let sum = s.iter().map(|&c| c.to_digit(10).unwrap()).sum::<u32>();

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
