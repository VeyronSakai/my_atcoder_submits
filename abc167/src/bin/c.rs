use nalgebra::min;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: i32
    }

    let mut c: Vec<i32> = Vec::new();
    let mut a: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        input! {
            c_tmp: i32,
            a_tmp: [i32; m]
        }

        c.push(c_tmp);
        a.push(a_tmp);
    }

    let mut ans = std::i32::MAX;

    // bit全探索して参考書を買うパターンを全て調べ上げる
    for bit in 0..(1 << n) {
        let mut comprehension = vec![0; m];
        let mut money = 0;

        // どの参考書を買ったのか一つずつ調べる
        for i in 0..n {
            // 1が立っている場合はその参考書は買ったことにする
            if bit >> i & 1 == 1 {
                for j in 0..m {
                    comprehension[j] += a[i][j];
                }

                money += c[i];
            }
        }

        let mut ok = true;

        for val in &comprehension {
            if *val < x {
                ok = false;
            }
        }

        if ok {
            ans = min(ans, money);
        }
    }

    if ans == std::i32::MAX {
        println!("-1");
        return;
    }

    println!("{}", ans);
}
