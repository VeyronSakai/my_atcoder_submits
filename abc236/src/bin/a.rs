use proconio::input;
use proconio::fastout;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
        a: usize,
        b: usize,
    }

    let aa = S[a - 1];
    let bb = S[b - 1];

    S[a - 1] = bb;
    S[b - 1] = aa;

    let ans: String = S.iter().collect();
    println!("{}", ans);
}
