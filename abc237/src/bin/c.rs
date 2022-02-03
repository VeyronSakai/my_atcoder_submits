use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let s: Vec<char> = s.chars().collect();

    let mut l = 0;
    let mut r = s.len() as i32 - 1;

    let length = s.len() as i32 - 1;

    // 左から見ていって最後にaが出てくる場所を調べる。最初の文字がaでなければ-1がlに入る
    for i in 0..length - 1 {
        if s[i as usize] == 'a' {
            continue;
        }

        l = i;
        break;
    }

    // 右から見ていって最後にaが出てくる場所を調べる。最初の文字がa出なければ s.len() がrに入る
    for i in length..0 {
        if s[i as usize] == 'a' {
            continue;
        }

        r = i;
        break;
    }

    // 文字列を切り出す
    let str = if *s.last().unwrap() != 'a' {
        &s[l as usize..r as usize + 1]
    } else {
        &s[l as usize..r as usize]
    };

    println!("{:?}", str);

    for i in 0..str.len() {
        if str[i] != str[str.len() - 1 - i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
