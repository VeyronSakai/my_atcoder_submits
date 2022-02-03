use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String
    }

    let s1 = s.trim_matches(|c| c == 'a').to_string();
    let t1 = s1.chars().rev().collect::<String>();

    if s1 == t1 {
        let s2 = s.chars().take_while(|&c| c == 'a').count();
        let t2 = s.chars().rev().take_while(|&c| c == 'a').count();

        if s2 <= t2 {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    } else {
        println!("No");
        return;
    }
}
