use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = if s == "ABC" { "ARC" } else if s == "ARC" { "ABC" } else { unreachable!() };

    println!("{}", ans);
}
