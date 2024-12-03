use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    // part one
    let (left_col, right_col) = read_and_sort_pairs("input.txt");
    if let Ok(res) = distance_pairs(left_col, right_col) {
        println!("day 1 res: {:?}", res);
    }
    // part two
    let (left, similarity_map) = read_list_and_similarity("input.txt");
    let res = compute_similarity(left, similarity_map);
    println!("day 2 res: {:?}", res);
}

fn compute_similarity(left: Vec<u64>, m: HashMap<u64, usize>) -> u64 {
    let mut res: u64 = 0;
    for word in left {
        // dbg!(key, value);
        if let Some(count) = m.get(&word) {
            let tmp: u64 = *count as u64;
            res += word * tmp;
        }
    }
    res
}

fn distance_pairs(left: Vec<u64>, right: Vec<u64>) -> Result<u64, io::Error> {
    let mut res: u64 = 0;
    if left.len() != right.len() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "left and right aren't of same length",
        ))
    } else {
        for i in 0..left.len() {
            res += left[i].abs_diff(right[i]);
        }
        Ok(res)
    }
}

fn read_and_sort_pairs(filename: &str) -> (Vec<u64>, Vec<u64>) {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));
    let mut left_col: Vec<u64> = Vec::new();
    let mut right_col: Vec<u64> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 2 {
            left_col.push(words[0].to_string().parse::<u64>().unwrap());
            right_col.push(words[1].to_string().parse::<u64>().unwrap());
        } else {
            eprintln!("Skipping malformed line: {}", line);
        }
    }
    left_col.sort();
    right_col.sort();
    (left_col, right_col)
}

fn read_list_and_similarity(filename: &str) -> (Vec<u64>, HashMap<u64, usize>) {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));
    let mut left_col: Vec<u64> = Vec::new();
    let mut right_col: Vec<u64> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 2 {
            left_col.push(words[0].to_string().parse::<u64>().unwrap());
            right_col.push(words[1].to_string().parse::<u64>().unwrap());
        } else {
            eprintln!("Skipping malformed line: {}", line);
        }
    }
    let mut m: HashMap<u64, usize> = HashMap::new();
    for word in right_col {
        *m.entry(word).or_default() += 1;
    }
    (left_col, m)
}

#[test]
fn test_distance_of_sorted_pairs() {
    let expected: u64 = 11;
    let (left_col, right_col) = read_and_sort_pairs("src/test.txt");

    if let Ok(res) = distance_pairs(left_col, right_col) {
        assert_eq!(expected, res);
    }
}

#[test]
fn test_similarity() {
    let expected: u64 = 31;
    let (left, similarity_map) = read_list_and_similarity("src/test.txt");
    let res = compute_similarity(left, similarity_map);
    assert_eq!(expected, res);
}
