use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        c: char,
    }

    // get next char
    println!("{}", char::from_u32(c as u32 + 1).unwrap());
}
