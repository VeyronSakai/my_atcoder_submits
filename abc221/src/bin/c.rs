use nalgebra::max;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }

    let mut ans = 0;

    let k = n.len();

    for bit in 0..1 << k {
        // 分離する
        let mut a: Vec<u64> = Vec::new();
        let mut b: Vec<u64> = Vec::new();

        for i in 0..k {
            if bit >> i & 1 == 1 {
                a.push(n[i].to_digit(10).unwrap() as u64);
            } else {
                b.push(n[i].to_digit(10).unwrap() as u64);
            }
        }

        // どちらかが空の場合は何もしない
        if a.is_empty() || b.is_empty() {
            continue;
        }

        // 昇順にソート
        a.sort();
        b.sort();

        // 0が一番大きい数字の場合は何もしない
        if *a.last().unwrap() == 0 || *b.last().unwrap() == 0 {
            continue;
        }

        let mut a_num = 0;
        let mut b_num = 0;

        let mut ten = 1;

        for i in 0..a.len() {
            a_num += a[i] * ten;
            ten *= 10;
        }

        ten = 1;
        for i in 0..b.len() {
            b_num += b[i] * ten;
            ten *= 10;
        }

        ans = max(ans, a_num * b_num);
    }

    println!("{}", ans);
}
