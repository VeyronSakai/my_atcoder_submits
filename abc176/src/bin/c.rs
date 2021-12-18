use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut max_step = 0_i64;

    let mut ans = 0_i64;

    for i in 0..n {
        max_step = max_step.max(a[i]);

        ans += max_step - a[i];
    }

    println!("{}", ans);
}
