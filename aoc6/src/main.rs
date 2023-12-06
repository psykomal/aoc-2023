use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut times: Vec<u64> = Vec::new();
    let mut dists: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let (first, rest) = line.split_once(':').unwrap() {
            if first == "Time" {
                times.push(
                    rest.split_whitespace()
                        .fold("".to_string(), |acc, x| acc + x)
                        .parse::<u64>()
                        .unwrap(),
                );
            } else if first == "Distance" {
                dists.push(
                    rest.split_whitespace()
                        .fold("".to_string(), |acc, x| acc + x)
                        .parse::<u64>()
                        .unwrap(),
                );
            }
        }
    }

    let mut prod: u64 = 1;
    let N = times.len();

    for i in 0..N {
        let time = times[i] as f64;
        let dist = dists[i] as f64;

        let mut left = ((time - (time.powi(2) - 4.0 * dist).sqrt()) / 2.0).ceil() as u64;
        let mut right = ((time + (time.powi(2) - 4.0 * dist).sqrt()) / 2.0).floor() as u64;

        if left * (time as u64 - left) == dist as u64 {
            left += 1;
        }
        if right * (time as u64 - right) == dist as u64 {
            right -= 1;
        }

        prod *= (right - left + 1);
    }

    println!("Prod : {}", prod);

    Ok(())
}

// Part 1
// for line in reader.lines() {
//     let line = line.unwrap();
//     if let (first, rest) = line.split_once(':').unwrap() {
//         if first == "Time" {
//             times.append(
//                 rest.split_whitespace()
//                     .map(|x| x.parse::<u64>().unwrap())
//                     .collect::<Vec<u64>>()
//                     .as_mut(),
//             );
//         } else if first == "Distance" {
//             dists.append(
//                 rest.split_whitespace()
//                     .map(|x| x.parse::<u64>().unwrap())
//                     .collect::<Vec<u64>>()
//                     .as_mut(),
//             );
//         }
//     }
// }
