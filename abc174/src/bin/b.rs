use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        v: [(i64, i64); n]
    }

    let mut ans = 0;

    for &(x, y) in &v {
        let length = ((x * x + y * y) as f64).sqrt();
        if length <= d {
            ans += 1;
        }
    }

    println!("{}", ans);
}
