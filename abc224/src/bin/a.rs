use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let tmp = &s[(s.len() - 2)..(s.len())];

    if tmp == "er" {
        println!("er");
        return;
    }

    let tmp2 = &s[(s.len() - 3)..(s.len())];

    if tmp2 == "ist" {
        println!("ist");
        return;
    }
}
