use num_traits::abs;
use permutohedron::LexicalPermutation;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
        q: [i32; n],
    }


    let mut sample = (1..=n).collect::<Vec<_>>().iter().map(|&x| x as i32).collect::<Vec<i32>>();
    let mut count = 0;

    let mut p_count: i32 = 0;
    let mut q_count: i32 = 0;

    loop {
        count += 1;

        if p == sample {
            p_count = count;
        }

        if q == sample {
            q_count = count;
        }

        if !sample.next_permutation() {
            break;
        }
    }

    println!("{}", (p_count - q_count).abs());
}
