use im_rc::HashSet;
use proconio::input;
use proconio::fastout;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut set: HashSet<char> = HashSet::new();

    for val in &s {
        set.insert(*val);
    }

    if set.len() == 2 {
        println!("3");
    }

    if set.len() == 3 {
        println!("6");
    }

    if set.len() == 1 {
        println!("1");
    }
}
