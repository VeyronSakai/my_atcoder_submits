use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut k: Vec<i32> = Vec::new();
    let mut s: Vec<Vec<i32>> = Vec::new();

    for _ in 0..m {
        input! {
            k_i: i32,
            s_i: [i32; k_i]
        }

        k.push(k_i);
        s.push(s_i);
    }

    input! {
        p: [i32; m],
    }

    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut ok = true;

        for i in 0..m {
            let mut switch_num = 0;

            for s in &s[i] {
                if 1 & (bit >> *s - 1) == 1
                {
                    switch_num += 1;
                }
            }

            if switch_num % 2 != p[i] {
                ok = false;
            }
        }

        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
