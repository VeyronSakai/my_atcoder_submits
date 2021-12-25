use nalgebra::max;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        conditions: [(i32, i32); m],
        k: usize,
        choice: [(usize, usize); k]
    }

    let mut ans = 0;

    // bit全探索を行う。
    // 各人がどちらの皿にボールを乗せるか、を全通り調べる
    for bitset in 0..(1 << k) {
        let mut dishes = vec![false; n];

        // 一人ずつ調べ上げる
        for i in 0..k {
            // bitが0ならCに、1ならDにボールを置いたことにする
            if bitset >> i & 1 == 0 {
                dishes[choice[i].0 - 1] = true;
            } else {
                dishes[choice[i].1 - 1] = true;
            }
        }

        let mut count = 0;
        // 最後に皿の上を調べて満たされている条件の数を調べる
        for i in 0..m {
            if dishes[(conditions[i].0 - 1) as usize] && dishes[(conditions[i].1 - 1) as usize] {
                count += 1;
            }
        }

        ans = max(ans, count);
    }

    println!("{}", ans);
}
