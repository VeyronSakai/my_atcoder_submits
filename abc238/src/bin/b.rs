use std::cmp::max;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut array = [false; 361];

    array[0] = true;
    array[360] = true;

    let mut cur: usize = 0;

    for i in 0..a.len() {
        cur += a[i];
        cur = cur % 360;
        array[cur] = true;
    }

    let mut count = 0;
    let mut ans = 0;

    for i in 0..361 {
        count += 1;
        if array[i] {
            ans = max(ans, count);
            count = 0;
        }
    }

    println!("{}", ans);
}
