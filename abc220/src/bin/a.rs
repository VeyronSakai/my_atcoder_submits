use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    for v in a..=b {
        if v % c == 0 {
            println!("{}", v);
            return;
        }
    }

    println!("{}", -1);
}
