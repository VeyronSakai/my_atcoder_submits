use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        x: f32,
    }

    // 四捨五入
    let ans = x.round() as i32;

    println!("{}", ans);
}
