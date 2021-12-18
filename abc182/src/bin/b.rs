use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut max_gcd = 0;
    let mut ans = -1;

    for i in 2..1001 {
        let mut tmp_gcd = 0;

        for &val in &a {
            if val % i == 0 {
                tmp_gcd += 1;
            }
        }

        if max_gcd < tmp_gcd {
            ans = i;
            max_gcd = tmp_gcd;
        }
    }

    println!("{}", ans);
}
