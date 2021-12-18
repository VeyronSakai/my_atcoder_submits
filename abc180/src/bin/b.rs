use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }

    let abs_x: Vec<i64> = x.iter().map(|&x| x.abs()).collect();

    let manhattan: i64 = abs_x.iter().sum();
    println!("{}", manhattan);

    let euclid = (x.iter().map(|&x| x.pow(2)).sum::<i64>() as f64).sqrt();
    println!("{}", euclid);

    let cheby_chef = x.iter().map(|&x| x.abs()).max().unwrap();
    println!("{}", cheby_chef);
}
