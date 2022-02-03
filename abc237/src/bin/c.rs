use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let s: Vec<char> = s.chars().collect();

    let mut r = s.len() as i32 - 1;

    let length = s.len() as i32;

    // 右から見ていって最後にaが出てくる場所を調べる。最初の文字がa出なければ s.len() がrに入る
    for i in (0..length).rev() {
        if s[i as usize] == 'a' {
            continue;
        }

        r = i;

        break;
    }

    let str = &s[0..r as usize + 1];
    println!("{:?}", str);

    for i in 0..str.len() {
        if str[i] != str[str.len() - 1 - i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
