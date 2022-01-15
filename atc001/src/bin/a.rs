use libm::ilogb;
use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        h: i32,
        w: i32,
        c: [String;h]
    }

    let cell: Vec<Vec<char>> = c.iter().map(|x| x.chars().collect()).collect();

    // sのあるセルから出発する
    let mut s_h = 0_i32;
    let mut s_w = 0_i32;

    for i in 0..h {
        for j in 0..w {
            if cell[i as usize][j as usize] == 's' {
                s_h = i;
                s_w = j;
            }
        }
    };

    // println!("{}, {}", s_h, s_w)

    // 上下左右が道だったら走査する
    if dfs(s_h, s_w, h, w, &cell) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(cur_h: i32, cur_w: i32, h: i32, w: i32, cells: &Vec<Vec<char>>) -> bool {
    if cells[cur_h as usize][cur_w as usize] == 'g' {
        return true;
    }

    if cur_h - 1 >= 0 {
        if cells[(cur_h - 1) as usize][cur_w as usize] == '#' {
            return false;
        }

        return dfs(cur_h - 1, cur_w, h, w, cells);
    } else if cur_h + 1 < h {
        if cells[(cur_h + 1) as usize][cur_w as usize] == '#' {
            return false;
        }
        return dfs(cur_h - 1, cur_w, h, w, cells);
    } else if cur_w - 1 >= 0 {
        if cells[cur_h as usize][(cur_w - 1) as usize] == '#' {
            return false;
        }
        return dfs(cur_h, cur_w - 1, h, w, cells);
    } else if cur_w + 1 < w {
        if cells[cur_h as usize][(cur_w + 1) as usize] == '#' {
            return false;
        }
        return dfs(cur_h, cur_w + 1, h, w, cells);
    }

    return false;
}
