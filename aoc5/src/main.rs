use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil: Vec<(u64, u64, u64, u64)> = Vec::new();
    let mut soil_to_fert: Vec<(u64, u64, u64, u64)> = Vec::new();
    let mut fert_to_water: Vec<(u64, u64, u64, u64)> = Vec::new();
    let mut water_to_light: Vec<(u64, u64, u64, u64)> = Vec::new();
    let mut light_to_temp: Vec<(u64, u64, u64, u64)> = Vec::new();
    let mut temp_to_hum: Vec<(u64, u64, u64, u64)> = Vec::new();
    let mut hum_to_loc: Vec<(u64, u64, u64, u64)> = Vec::new();

    let mut state = 0;

    // Read Input
    for line in reader.lines() {
        let line = line?;

        if line.trim() == "" {
            continue;
        }

        if let Some((first, rest)) = line.split_once(":") {
            // handle seeds
            if first == "seeds" {
                seeds = rest
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            } else if first == "seed-to-soil map" {
                state = 1;
            } else if first == "soil-to-fertilizer map" {
                state = 2;
            } else if first == "fertilizer-to-water map" {
                state = 3;
            } else if first == "water-to-light map" {
                state = 4;
            } else if first == "light-to-temperature map" {
                state = 5;
            } else if first == "temperature-to-humidity map" {
                state = 6;
            } else if first == "humidity-to-location map" {
                state = 7;
            }
            continue;
        }

        // println!("{:?}", first);
        // collect numbers
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let (x, y, z, w) = (
            numbers[1],
            numbers[1] + numbers[2] - 1,
            numbers[0],
            numbers[0] + numbers[2] - 1,
        );

        if state == 1 {
            seed_to_soil.push((x, y, z, w));
        } else if state == 2 {
            soil_to_fert.push((x, y, z, w));
        } else if state == 3 {
            fert_to_water.push((x, y, z, w));
        } else if state == 4 {
            water_to_light.push((x, y, z, w));
        } else if state == 5 {
            light_to_temp.push((x, y, z, w));
        } else if state == 6 {
            temp_to_hum.push((x, y, z, w));
        } else if state == 7 {
            hum_to_loc.push((x, y, z, w));
        }
    }

    // Part 2 : new seeds
    let mut new_seeds = seeds
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1] - 1))
        .collect::<Vec<(u64, u64)>>();
    println!("{:?}", new_seeds);

    state = 0;

    while state < 7 {
        println!("\nnew_seeds : {:?}", new_seeds);
        println!();
        if state == 0 {
            seed_to_soil.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &seed_to_soil))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", seed_to_soil);
            state = 1;
        } else if state == 1 {
            soil_to_fert.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &soil_to_fert))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", soil_to_fert);
            state = 2;
        } else if state == 2 {
            fert_to_water.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &fert_to_water))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", fert_to_water);
            state = 3;
        } else if state == 3 {
            water_to_light.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &water_to_light))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", water_to_light);
            state = 4;
        } else if state == 4 {
            light_to_temp.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &light_to_temp))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", light_to_temp);
            state = 5;
        } else if state == 5 {
            temp_to_hum.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &temp_to_hum))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", temp_to_hum);
            state = 6;
        } else if state == 6 {
            hum_to_loc.sort();
            new_seeds = new_seeds
                .iter()
                .map(|(n_l, n_r)| mapper_new(*n_l, *n_r, &hum_to_loc))
                .collect::<Vec<Vec<(u64, u64)>>>()
                .concat();
            println!("{:?}", hum_to_loc);
            state = 7;
        }
    }

    new_seeds.sort();
    println!("Answer: {:?}", new_seeds);

    Ok(())
}

fn mapper_new(n_l: u64, n_r: u64, map: &Vec<(u64, u64, u64, u64)>) -> Vec<(u64, u64)> {
    // println!("\n\nmap {:?}   query: {:?}", map, (n_l, n_r));
    let (mut l1, mut r1) = (0, map.len() - 1);
    while l1 < r1 {
        let m = (l1 + r1) / 2;

        let (x, y, z, w) = map[m];

        if y >= n_l {
            r1 = m;
        } else {
            l1 = m + 1;
        }
    }

    let (mut l2, mut r2) = (0, map.len() - 1);
    while l2 < r2 {
        let m = (l2 + r2) / 2;

        let (x, y, z, w) = map[m];

        if y >= n_r {
            r2 = m;
        } else {
            l2 = m + 1;
        }
    }

    let mut n_l = n_l;
    let mut n_r = n_r;
    let mut ans = Vec::new();
    // println!("l1 l2 {} {}", l1, l2);

    for i in l1..=l2 {
        let (x, y, z, w) = map[i];
        if n_l > y {
            ans.push((n_l, n_r));
            break;
        }
        if n_l < x {
            ans.push((n_l, n_r.min(x - 1)));
            n_l = x;
        }
        if n_l <= y {
            let d = z as i64 - x as i64;
            ans.push(((n_l as i64 + d) as u64, (n_r.min(y) as i64 + d) as u64));
            if n_r <= y {
                break;
            }
            n_l = y + 1;
        }
        if i == l2 {
            ans.push((n_l, n_r));
            break;
        }
    }

    // println!("ans {:?}", ans);
    ans
}

fn mapper(n: u64, map: &Vec<(u64, u64, u64, u64)>) -> u64 {
    let (mut l, mut r) = (0, map.len() - 1);

    while l < r {
        let m = (l + r) / 2;

        let (x, y, z, w) = map[m];

        if y >= n {
            r = m;
        } else {
            l = m + 1;
        }
    }

    let (x, y, z, w) = map[l];
    if x <= n && n <= y {
        return z + n - x;
    }
    n
}
