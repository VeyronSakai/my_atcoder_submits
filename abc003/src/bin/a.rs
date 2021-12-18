use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut ans: f32 = 0.0;

    for i in 1..n + 1 {
        ans = ans + i as f32 / n as f32;
    }

    println!("{}", ans * 10000 as f32);
}
