use permutohedron::LexicalPermutation;
use proconio::input;

pub struct Town {
    pub x: i64,
    pub y: i64,
}

fn main() {
    input! {
        n: usize,
        vector: [(i64,i64); n]
    }

    let mut sample: Vec<usize> = (0..n).collect();

    let mut towns: Vec<Town> = Vec::new();

    for i in 0..n {
        towns.push(Town {
            // id: i as i32,
            x: vector[i].0,
            y: vector[i].1,
        });
    }

    let mut sum = 0_f64;
    let mut count = 0;

    loop {
        count += 1;

        for i in 0..n - 1 {
            let x = towns[sample[i + 1]].x - towns[sample[i]].x;
            let y = towns[sample[i + 1]].y - towns[sample[i]].y;

            sum += ((x * x + y * y) as f64).sqrt();
        }

        if !sample.next_permutation() {
            break;
        }
    }

    println!("{}", sum / count as f64);
}
