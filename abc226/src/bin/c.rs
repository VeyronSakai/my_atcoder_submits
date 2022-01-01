use std::cmp::min;
use im_rc::HashSet;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut t: Vec<i64> = Vec::new();
    let mut k: Vec<i64> = Vec::new();
    let mut a: Vec<Vec<usize>> = Vec::new();

    for _ in 0..n {
        input! {
            input_t: i64,
            input_k:i64,
            input_a:[usize; input_k],
        }

        t.push(input_t);
        k.push(input_k);
        a.push(input_a);
    }

    let mut memo_time = vec![0; n];
    let mut memo_skill: Vec<HashSet<i64>> = vec![HashSet::new(); n];

    memo_time[0] = t[0];
    memo_skill[0].insert(0); // 0番目の技は習得済み

    // 技を習得するのにかかる最小時間を配列にメモしていく
    // i番目技を習得したときにすでに習得している他の技もメモしていく
    for i in 1..n {
        let mut min_time = i64::MAX;

        if a[i].is_empty() {
            min_time = 0;
        }

        for val in &a[i] {
            min_time = min(min_time, memo_time[*val - 1]);
        }

        memo_time[i] = min_time + t[i];
    }

    for val in &a[n - 1] {}
}
