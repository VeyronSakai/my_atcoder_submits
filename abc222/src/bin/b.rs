use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i32,
        a: [i32; n],
    }

    let ans = a.iter().filter(|&x| *x < p).count();

    println!("{}", ans);
}
