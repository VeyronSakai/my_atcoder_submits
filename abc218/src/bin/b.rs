use ascii::ToAsciiChar;
use proconio::input;

fn main() {
    input! {
        p: [u32; 26],
    }

    let tmp = 'a' as u32 - 1;

    for &value in &p {
        let char = (value + tmp).to_ascii_char().unwrap().to_ascii_lowercase();
        print!("{}", char);
    }
}
