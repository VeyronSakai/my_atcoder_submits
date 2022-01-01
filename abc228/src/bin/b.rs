use std::collections::HashSet;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let mut set: HashSet<usize> = HashSet::new();

    let mut s = x;

    loop {
        if set.contains(&s) {
            break;
        }

        set.insert(s);

        s = a[s - 1];
    }

    println!("{}", set.len());
}
