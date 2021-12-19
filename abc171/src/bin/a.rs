use proconio::input;

fn main() {
    input! {
        a: char,
    }

    let ans = match a {
        'a'..='z' => 'a',
        'A'..='Z' => 'A',
        _ => unreachable!()
    };

    println!("{}", ans);
}
