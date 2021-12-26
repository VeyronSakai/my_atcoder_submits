use im_rc::HashSet;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut set: HashSet<Vec<i64>> = HashSet::new();

    for _ in 0..n {
        input! {
            l: i32,
            a: [i64; l]
        }

        set.insert(a);
    }

    println!("{}", set.len());
}
