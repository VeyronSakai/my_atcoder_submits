use std::cmp::Ordering::{Greater, Less};
use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
        a: [usize; n]
    );

    // 二分探索 partition_part の代わり
    let i = a.binary_search_by(|&x| if x < k { Less } else { Greater }).unwrap_or_else(|x| x);

    if i == n {
        println!("-1");
    } else {
        println!("{}", i);
    }
}
