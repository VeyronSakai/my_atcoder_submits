use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }

    let mut q = vec![0; n];

    for i in 0..n {
        q[(p[i] - 1) as usize] = i + 1;
    }

    for &v in &q {
        print!("{}", v);
        print!(" ");
    }
}
