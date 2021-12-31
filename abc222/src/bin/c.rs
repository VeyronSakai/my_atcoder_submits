use std::ptr::null;
use proconio::input;

pub struct Person {
    pub id: i32,
    pub ranking: i32,
    pub hands: Vec<char>,
    pub win_num: i32,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [String; 2*n]
    }

    let mut persons: Vec<Person> = Vec::new();

    for i in 0..2 * n {
        let p = Person {
            id: i as i32 + 1,
            ranking: i as i32 + 1,
            hands: a[i].chars().collect(),
            win_num: 0,
        };

        persons.push(p);
    }

    // 試合をする
    for i in 0..m {
        for j in 0..n {
            // 順位がjの人とj+1の人が戦う
            let (result1, result2) = janken(persons[2 * j].hands[i], persons[2 * j + 1].hands[i]);

            if result1 {
                persons[2 * j].win_num += 1;
            }

            if result2 {
                persons[2 * j + 1].win_num += 1;
            };
        }

        // idでソート
        persons.sort_by_key(|x| x.id);

        // 勝ち数で安定ソート
        persons.sort_by(|x, y| x.win_num.cmp(&y.win_num).reverse());

        // ランキングを更新
        for k in 0..2 * n {
            persons[k].ranking = k as i32 + 1;
        }

        // ランキングでソート
        persons.sort_by_key(|x| x.ranking)
    }

    for i in 0..2 * n {
        println!("{}", persons[i].id);
    }
}

fn janken(hand1: char, hand2: char) -> (bool, bool) {
    if hand1 == 'G' && hand2 == 'C' {
        // 勝ち
        return (true, false);
    }

    if hand1 == 'G' && hand2 == 'P' {
        // 負け
        return (false, true);
    }

    if hand1 == 'C' && hand2 == 'P' {
        // 勝ち
        return (true, false);
    }

    if hand1 == 'C' && hand2 == 'G' {
        // 負け
        return (false, true);
    }

    if hand1 == 'P' && hand2 == 'G' {
        // 勝ち
        return (true, false);
    }

    if hand1 == 'P' && hand2 == 'C' {
        // 負け
        return (false, true);
    }

    // どれでもない場合はあいこ
    return (false, false);
}
