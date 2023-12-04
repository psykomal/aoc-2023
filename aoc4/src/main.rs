use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut scores = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let sec = line.split(":").collect::<Vec<&str>>()[1].trim();

        let sec_list = sec.split("|").collect::<Vec<&str>>();

        let first_str = sec_list[0].trim();
        let second_str = sec_list[1].trim();

        // println!("first str: {} | second_str: {}", first_str, second_str);

        let first_set = first_str
            .split(" ")
            .filter_map(|x| {
                if x.trim().is_empty() {
                    None
                } else {
                    Some(x.parse::<u64>().unwrap())
                }
            })
            .collect::<HashSet<u64>>();

        let second_set = second_str
            .split(" ")
            .filter_map(|x| {
                if x.trim().is_empty() {
                    None
                } else {
                    Some(x.parse::<u64>().unwrap())
                }
            })
            .collect::<HashSet<u64>>();

        let intersection = first_set.intersection(&second_set).count();
        // println!("{} {}", i, intersection);
        for j in 1..intersection + 1 {
            if i + j < scores.len() {
                scores[i + j] += scores[i];
            } else {
                break;
            }
        }
    }

    println!("vec {:?}", scores);
    println!("ans {}", scores.iter().sum::<usize>());

    Ok(())
}
