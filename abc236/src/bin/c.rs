use std::collections::HashMap;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut map: HashMap<String, bool> = HashMap::new();

    for stop in &t {
        map.insert(stop.clone(), true);
    }

    for stop in &s {
        let ans = map.get(stop).unwrap_or(&false);

        if *ans {
            println!("Yes");
        }else{
            println!("No");
        }
    }
}
