use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn rank(cards: String) -> u64 {
    let mut rank = 0;
    let mut map = HashMap::new();

    for c in cards.chars() {
        map.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    // For Part 2
    let cnt_j = *map.get(&'J').unwrap_or(&0);
    // delete J
    map.remove(&'J');

    let mut cnts = map.values().map(|k| *k).collect::<Vec<usize>>();
    cnts.sort();
    let cnts_len = cnts.len();

    if cnts_len != 0 {
        cnts[cnts_len - 1] += cnt_j;
    } else {
        cnts.push(cnt_j);
    }

    let cnts_str = cnts
        .iter()
        .map(|k| k.to_string())
        .fold("".to_string(), |acc, k| acc + &k);
    let rank = match cnts_str.as_str() {
        "5" => 1,
        "14" => 2,
        "23" => 3,
        "113" => 4,
        "122" => 5,
        "1112" => 6,
        "11111" => 7,
        _ => 8,
    };

    rank as u64
}

fn card_rank(card: char) -> u64 {
    let ranking = "AKQT98765432J";

    ranking.chars().position(|c| c == card).unwrap() as u64
}

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut vec: Vec<(String, u64)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let split = line
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        let card = split[0].clone();
        let bid = split[1].parse::<u64>().unwrap();
        vec.push((card, bid));
    }

    vec.sort_by(|a, b| {
        let ranka = rank(a.0.clone());
        let rankb = rank(b.0.clone());

        if ranka == rankb {
            for i in 0..a.0.len() {
                let carda = a.0.chars().nth(i).unwrap();
                let cardb = b.0.chars().nth(i).unwrap();
                let rank_carda = card_rank(carda);
                let rank_cardb = card_rank(cardb);
                if rank_carda > rank_cardb {
                    return std::cmp::Ordering::Less;
                } else if rank_carda < rank_cardb {
                    return std::cmp::Ordering::Greater;
                }
            }
        } else if ranka > rankb {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }

        return std::cmp::Ordering::Greater;
    });

    let mut ans: u64 = 0;
    for i in 0..vec.len() {
        ans += vec[i].1 * (i as u64 + 1);
    }

    println!("{}", ans);
    println!("{:?}", vec);

    Ok(())
}
