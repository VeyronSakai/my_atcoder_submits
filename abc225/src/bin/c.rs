use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[i64; m]; n]
    }

    let mut pre_head = 0;

    let mut line = 0;
    let mut column = 0;

    // 左上の要素の行と列を調べる
    for j in 1..=7 {
        if (b[0][0] - j) % 7 == 0 && (b[0][0] - j) / 7 >= 0 {
            line = (b[0][0] - j) / 7;
            column = j;
            break;
        }
    }

    if column + m as i64 - 1 > 7 {
        println!("No");
        return;
    }

    //1行ずつ見ていく
    for i in 0..n {
        // 行の頭の数字をチェック。差が7であるか。
        if b[i][0] - pre_head != 7 && i != 0 { // i==0の時はこのチェックはパス
            println!("No");
            return;
        }

        // 行の頭の数字をメモ
        pre_head = b[i][0];

        // 1文字ずつ見ていく
        for j in 1..m {
            // 差が1であるかのチェック
            if b[i][j] - b[i][j - 1] != 1 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
