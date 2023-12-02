use std::fs::File;
use std::io::{self, BufRead};

enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn get_color(line: &str) -> Color {
    match line {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Unknown color"),
    }
}

fn get_counts(line: &str) -> Vec<usize> {
    let mut counts = vec![0; 3];

    let comma_split = line.split(",").collect::<Vec<&str>>();

    for item in comma_split {
        let space_split = item.trim().split(" ").collect::<Vec<&str>>();
        let number = space_split[0].parse::<usize>().unwrap();
        let color = get_color(space_split[1]);
        counts[color as usize] = number;
    }

    counts
}

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        let game_str = line.split(":").collect::<Vec<&str>>()[1];
        let sub_games = game_str.trim().split(";").collect::<Vec<&str>>();

        let mut max_color_count = vec![0; 3];
        for sub_game in sub_games {
            let counts = get_counts(sub_game.trim());
            max_color_count[Color::Red as usize] =
                max_color_count[Color::Red as usize].max(counts[Color::Red as usize]);
            max_color_count[Color::Green as usize] =
                max_color_count[Color::Green as usize].max(counts[Color::Green as usize]);
            max_color_count[Color::Blue as usize] =
                max_color_count[Color::Blue as usize].max(counts[Color::Blue as usize]);
        }
        sum += max_color_count[Color::Red as usize]
            * max_color_count[Color::Green as usize]
            * max_color_count[Color::Blue as usize];
    }

    println!("Sum: {}", sum);
    Ok(())
}

// Part 1
//
// fn main() -> io::Result<()> {
//     let file_path = "./src/input.txt";
//     let file = File::open(&file_path)?;
//     let reader = io::BufReader::new(file);

//     let mut sum = 0;
//     let mut count = 1;

//     for line in reader.lines() {
//         let line = line?;

//         let game_str = line.split(":").collect::<Vec<&str>>()[1];
//         let sub_games = game_str.trim().split(";").collect::<Vec<&str>>();

//         let mut valid_game = true;
//         for sub_game in sub_games {
//             let counts = get_counts(sub_game.trim());
//             if counts[Color::Red as usize] > MAX_RED
//                 || counts[Color::Green as usize] > MAX_GREEN
//                 || counts[Color::Blue as usize] > MAX_BLUE
//             {
//                 valid_game = false;
//                 break;
//             }
//         }
//         if valid_game {
//             sum += count;
//         }
//         count += 1;
//     }

//     println!("Sum: {}", sum);
//     Ok(())
// }
