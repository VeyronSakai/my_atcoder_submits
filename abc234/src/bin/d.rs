use std::cmp::Reverse;
use std::collections::BinaryHeap;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [i32; n]
    }

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for i in 0..n {
        if i < k - 1 {
            heap.push(Reverse(p[i]));
        } else if i == k - 1 {
            heap.push(Reverse(p[i]));
            println!("{}", heap.peek().unwrap().0);
        } else {
            if p[i] > heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(p[i]));
            }

            println!("{}", heap.peek().unwrap().0);
        }
    }
}
