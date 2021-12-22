use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ss = s.clone() + s.as_str();

    let length = s.len();

    let mut vec: Vec<String> = Vec::new();

    for i in 0..s.len() {
        let t = &ss[i..length + i];
        vec.push(t.to_string());
    }

    vec.sort();

    println!("{}", vec.first().unwrap());
    println!("{}", vec.last().unwrap());
}
