use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k :u64,
        mut a: Chars,
        mut b: Chars,
    }

    a.reverse();
    b.reverse();

    let mut new_a = 0_u64;
    let mut new_b = 0_u64;

    let mut a_k: u64 = 1;
    let mut b_k: u64 = 1;

    for &v in &a {
        new_a = new_a + v.to_digit(10).unwrap() as u64 * a_k;
        a_k = a_k * k;
    }

    for &v in &b {
        new_b = new_b + v.to_digit(10).unwrap() as u64 * b_k;
        b_k = b_k * k;
    }

    println!("{}", new_a * new_b);
}
