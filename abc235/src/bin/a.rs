use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        input: String,
    }

    let mut s: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let ans = (s[2] + s[1] + s[0]) * 100 + (s[0] + s[1] + s[2]) * 10 + s[0] + s[1] + s[2];

    println!("{}", ans);
}
