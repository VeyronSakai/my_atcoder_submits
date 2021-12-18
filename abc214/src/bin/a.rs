use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let ans: i32 = if n <= 125 { 4 } else if n <= 211 { 6 } else if n <= 214 { 8 } else { panic!() };

    println!("{}", ans);
}
