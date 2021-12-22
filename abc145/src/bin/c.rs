use std::cmp::Ordering;
use permutohedron::LexicalPermutation;
use proconio::input;

pub struct Town {
    pub id: i32,
    pub x: i64,
    pub y: i64,
}

impl PartialEq<Self> for Town {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for Town {}

impl PartialOrd<Self> for Town {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Town {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.id.cmp(&other.id);
    }
}

fn main() {
    input! {
        n: usize,
        vector: [(i64,i64); n]
    }

    let mut towns: Vec<Town> = Vec::new();

    for i in 0..n {
        towns.push(Town {
            id: i as i32,
            x: vector[i].0,
            y: vector[i].1,
        });
    }

    let mut sum = 0_f64;
    let mut count = 0;

    loop {
        count += 1;

        for i in 0..n - 1 {
            let x = towns[i + 1].x - towns[i].x;
            let y = towns[i + 1].y - towns[i].y;

            sum += ((x * x + y * y) as f64).sqrt();
        }

        if !towns.next_permutation() {
            break;
        }
    }

    println!("{}", sum / count as f64);
}
