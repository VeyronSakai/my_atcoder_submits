use std::collections::HashMap;
use proconio::input;
use proconio::fastout;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n]
    }

    // 順変換用のmapと逆変換用のmapを用意する
    let mut mp: HashMap<char, char> = HashMap::new();
    let mut reverse_mp: HashMap<char, char> = HashMap::new();
    let sample: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for i in 0..sample.len() {
        mp.insert(sample[i], x[i]);
        reverse_mp.insert(x[i], sample[i]);
    }

    for str in &mut s {
        for c in str {
            // 順変換
            *c = *(mp.get(c).unwrap());
        }
    }

    // ソートする
    s.sort();

    // 逆変換して元に戻してから結果を出力する
    for str in &mut s {
        for c in str {
            // 逆変換
            *c = *(reverse_mp.get(c).unwrap());
        }
    }

    for vec in &s {
        let ret = vec.iter().collect::<String>();
        println!("{}", ret);
    }
}
