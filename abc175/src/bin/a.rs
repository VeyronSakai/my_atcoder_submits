use nalgebra::max;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans = 0;
    let mut count = 0;

    let mut pre_char = '_';

    for char in s.chars() {
        if pre_char == char {
            count += 1;
            ans = max(ans, count);
        } else {
            pre_char = char;
            count = 1;
            ans = max(ans, count);
        }
    }

    println!("{}", ans);
}
