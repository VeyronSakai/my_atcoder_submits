use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    if n <= 125{
        println!("{}", 4);
    }else if n <= 211 {
        println!("{}", 6);
    }else if n <= 214 {
        println!("{}", 8);
    }
}
