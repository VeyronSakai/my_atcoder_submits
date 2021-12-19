use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        x: i32,
        s: Chars
    }

    let mut ans = x;

    for &char in &s {
        match char {
            'o' => ans += 1,
            'x' => {
                if ans > 0 {
                    ans -= 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
