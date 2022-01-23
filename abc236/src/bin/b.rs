use std::collections::BTreeSet;
use im_rc::HashMap;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; 4 * n - 1],
    }

    let mut map : HashMap<i32, i32> = HashMap::new();

    for v in &a {
        let mut count = *map.get(v).unwrap_or(&0);
        count += 1;
        map.insert(*v, count);
    }

    for v in 1..=n {
        let key = v as i32;
        if *map.get(&key).unwrap() == 3{
            println!("{}", v);
        }
    }
}
