use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        input: [(i64, i64); n],
    }

    let mut ans = 0;

    // 3点選ぶ
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                // 3点が一直線上に無いか確かめる
                let vec_i_x = input[i].0 - input[k].0;
                let vec_i_y = input[i].1 - input[k].1;

                let vec_j_x = input[j].0 - input[k].0;
                let vec_j_y = input[j].1 - input[k].1;

                // 外積を計算して0なら三角形が成立しない
                if vec_i_x * vec_j_y - vec_i_y * vec_j_x != 0 {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
