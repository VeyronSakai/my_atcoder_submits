use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        x: u64
    }

    let sum: u64 = a.iter().sum();

    let p = x / sum;

    let mut sum_b = p * sum;

    let mut ans = p * n as u64;

    for i in 0..n {
        sum_b += a[i];
        ans += 1;

        if sum_b > x {
            println!("{}", ans);
            return;
        }
    }
}
