use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [i32; n]
    }

    // 4ab + 3a + 3b <= 1000
    // b = 1の場合
    // 7a + 3 <= 1000
    // aの値の範囲は
    // a <= 997 / 7 < 143
    // 同様に b < 143

    // 全探索すると計算量は O(10^6)なので間に合う

    let mut count = 0;

    for i in 0..n {
        tmp(&mut count, s[i])
    }

    println!("{}", n as i32 - count);
}

fn tmp(count: &mut i32, s: i32) {
// 全てのa, bについて全探索
    for a in 1..143 { // aとbは自然数であることに注意
        for b in 1..143 {
            if 4 * a * b + 3 * a + 3 * b == s {
                *count = *count + 1;
                return;
            }
        }
    }
}