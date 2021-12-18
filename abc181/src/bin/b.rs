use proconio::input;

fn main() {
    input! {
        n: usize,
        v : [(i64, i64); n]
    }

    let mut ans = 0_i64;

    for i in 0..n {
        ans += (v[i].1 + v[i].0) * (v[i].1 - v[i].0 + 1) / 2;
    }

    println!("{}", ans);
}
