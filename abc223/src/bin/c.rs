use proconio::input;

fn main() {
    input! {
        n: usize,
        vec: [(i32, i32); n]
    }

    let a = vec.iter().map(|x| x.0).collect::<Vec<_>>();
    let b = vec.iter().map(|x| x.1).collect::<Vec<_>>();

    // 導火線の長さが最大でO(10^8) cm くらいになりそうなので、1秒ずつシミュレーションするとO(10^8)くらいの計算量になりそうでダメ

    // 累積和(cumulative sum)で行く
    // 左からi番目の導火線が燃え尽きる時間を記録する

    let mut left = vec![0_f32; n];
    let mut right = vec![0_f32; n];

    left[0] = a[0] as f32 / b[0] as f32;
    right[n - 1] = a[n - 1] as f32 / b[n - 1] as f32;

    for i in 1..n {
        left[i] = left[i - 1] + a[i] as f32 / b[i] as f32;
        right[n - i - 1] = right[n - i] + a[n - i - 1] as f32 / b[n - i - 1] as f32;
    }

    // ~~~|--|--====
    // ~~~--|--|====

    // の2パターンのみ
    
}
