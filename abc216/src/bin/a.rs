use proconio::input;

fn main() {
    input! {
        input: f32,
    }

    let f_num: i32 = ((input * 10.0) as i32) % 10;
    let int_num: i32 = input as i32;

    match f_num {
        0..=2 => {
            println!("{}", int_num.to_string() + "-");
        }
        3..=6 => {
            println!("{}", int_num);
        }
        7..=9 => {
            println!("{}", int_num.to_string() + "+");
        }
        _ => unreachable!(),
    }
}
