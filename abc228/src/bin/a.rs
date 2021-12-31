use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        s: i32,
        t: i32,
        x: i32,
    }

    // 日付を跨がないパターン
    if s < t {
        if s <= x && x < t {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    } else {
        // 日付を跨ぐパターン
        if (s <= x && x <= 23) || (0 <= x && x < t) {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    }
}
