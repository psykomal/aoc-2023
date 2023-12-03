use std::io::{self, BufRead};
use std::{collections::HashSet, fs::File};

fn iterate_matrix(
    matrix: &Vec<Vec<char>>,
    vis: &mut HashSet<(i32, i32)>,
    coords: (i32, i32),
) -> usize {
    let mut nums = Vec::new();
    let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);

    for i in vec![-1, 0, 1].iter() {
        for j in vec![-1, 0, 1].iter() {
            if *i != 0 || *j != 0 {
                let (x, y) = (coords.0 + i, coords.1 + j);

                if (x >= 0 && x < m)
                    && (y >= 0 && y < n)
                    && matrix[x as usize][y as usize].is_digit(10)
                    && !vis.contains(&(x, y))
                {
                    let mut k = y - 1;
                    // println!("k {} {}", k, k as usize);
                    while k >= 0 && matrix[x as usize][k as usize].is_digit(10) {
                        k -= 1;
                    }
                    // println!("k {} {}", k, k as usize);
                    k += 1;

                    let mut curr = 0;
                    // println!("k {} {}", k, k as usize);
                    while k < n && matrix[x as usize][k as usize].is_digit(10) {
                        let n = matrix[x as usize][k as usize].to_digit(10).unwrap();
                        curr = 10 * curr + n;
                        vis.insert((x, k));
                        k += 1;
                    }

                    // println!("curr: {}", curr);

                    nums.push(curr);
                }
            }
        }
    }

    if nums.len() == 2 {
        return nums[0] as usize * nums[1] as usize;
    }

    0
}

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    // construct matrix
    let mut matrix = Vec::new();
    for line in reader.lines() {
        matrix.push(line?.chars().collect::<Vec<char>>());
    }

    // Deque and BFS
    // expand the numbers and mark visited

    let mut sum: usize = 0;
    let mut vis: HashSet<(i32, i32)> = HashSet::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if matrix[i][j] == '*' {
                sum += iterate_matrix(&matrix, &mut vis, (i as i32, j as i32));
            }
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}

// Part 1
// fn iterate_matrix(
//     matrix: &Vec<Vec<char>>,
//     vis: &mut HashSet<(i32, i32)>,
//     coords: (i32, i32),
// ) -> usize {
//     let mut sum: usize = 0;
//     let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);

//     for i in vec![-1, 0, 1].iter() {
//         for j in vec![-1, 0, 1].iter() {
//             if *i != 0 || *j != 0 {
//                 let (x, y) = (coords.0 + i, coords.1 + j);

//                 if (x >= 0 && x < m)
//                     && (y >= 0 && y < n)
//                     && matrix[x as usize][y as usize].is_digit(10)
//                     && !vis.contains(&(x, y))
//                 {
//                     let mut k = y - 1;
//                     // println!("k {} {}", k, k as usize);
//                     while k >= 0 && matrix[x as usize][k as usize].is_digit(10) {
//                         k -= 1;
//                     }
//                     // println!("k {} {}", k, k as usize);
//                     k += 1;

//                     let mut curr = 0;
//                     // println!("k {} {}", k, k as usize);
//                     while k < n && matrix[x as usize][k as usize].is_digit(10) {
//                         let n = matrix[x as usize][k as usize].to_digit(10).unwrap();
//                         curr = 10 * curr + n;
//                         vis.insert((x, k));
//                         k += 1;
//                     }

//                     // println!("curr: {}", curr);

//                     sum += curr as usize;
//                 }
//             }
//         }
//     }

//     sum
// }

// fn main() -> io::Result<()> {
//     let file_path = "./src/input.txt";
//     let file = File::open(&file_path)?;
//     let reader = io::BufReader::new(file);

//     // construct matrix
//     let mut matrix = Vec::new();
//     for line in reader.lines() {
//         matrix.push(line?.chars().collect::<Vec<char>>());
//     }

//     // Deque and BFS
//     // expand the numbers and mark visited

//     let mut sum: usize = 0;
//     let mut vis: HashSet<(i32, i32)> = HashSet::new();

//     for (i, row) in matrix.iter().enumerate() {
//         for (j, c) in row.iter().enumerate() {
//             if matrix[i][j] != '.' && !matrix[i][j].is_digit(10) {
//                 sum += iterate_matrix(&matrix, &mut vis, (i as i32, j as i32));
//             }
//         }
//     }

//     println!("Sum: {}", sum);
//     Ok(())
// }
