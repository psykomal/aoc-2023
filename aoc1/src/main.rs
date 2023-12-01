use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    digit: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            digit: -1,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String, digit: i32) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert(TrieNode::new());
            current_node = next_node;
        }
        current_node.digit = digit;
    }

    fn search(&self, word: String) -> i32 {
        for i in 0..word.len() {
            let mut current_node = &self.root;

            for c in word.chars().skip(i) {
                if let Some(next_node) = current_node.children.get(&c) {
                    current_node = next_node;
                } else {
                    break;
                }

                if current_node.digit != -1 {
                    return current_node.digit;
                }
            }
        }
        -1
    }
}

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(Path::new(&file_path))?;
    let reader = io::BufReader::new(file);
    let mut sum: usize = 0;

    // Trie setup
    let mut trie = Trie::new();

    let mut setup = HashMap::<i32, Vec<&str>>::new();
    setup.insert(1, vec!["1", "one", "eno"]);
    setup.insert(2, vec!["2", "two", "owt"]);
    setup.insert(3, vec!["3", "three", "eerht"]);
    setup.insert(4, vec!["4", "four", "ruof"]);
    setup.insert(5, vec!["5", "five", "evif"]);
    setup.insert(6, vec!["6", "six", "xis"]);
    setup.insert(7, vec!["7", "seven", "neves"]);
    setup.insert(8, vec!["8", "eight", "thgie"]);
    setup.insert(9, vec!["9", "nine", "enin"]);

    for (digit, words) in setup {
        for word in words {
            trie.insert(word.to_string(), digit);
        }
    }

    // Main Computation of input

    for line in reader.lines() {
        let line = line?;

        let digit1 = trie.search(line.clone());
        sum += 10 * (digit1 as usize);

        let digit2 = trie.search(line.chars().rev().collect());
        sum += digit2 as usize;

        // println!("{} {} {}", line, digit1, digit2);
    }

    println!("Sum: {}", sum);
    Ok(())
}
