use num_traits::abs;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }

    let mut ans = 0;

    // nが3未満のときに注意！！
    if n < 3 {
        println!("{}", ans);
        return;
    }

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if l[i] == l[j] || l[j] == l[k] || l[k] == l[i] {
                    continue;
                }

                if l[i] + l[j] > l[k] && abs(l[i] - l[j]) < l[k]
                {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
