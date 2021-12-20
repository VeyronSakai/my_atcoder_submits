use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    match x {
        0..=39 => {
            println!("{}", 40 - x);
        }
        40..=69 => {
            println!("{}", 70 - x);
        }
        70..=89 => println!("{}", 90 - x),
        90..=100 => println!("expert"),
        _ => unreachable!()
    };
}
