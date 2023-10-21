use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_lines(filename: &str) -> Vec<String> {
    let mut result = vec![];
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip);
            }
        }
    }
    result
}

fn parse_input1(filename: &str) -> Vec<BTreeSet<char>> {
    let mut result = vec![];
    result.push(BTreeSet::new());
    let lines = get_lines(filename);
    for line in lines {
        if line == "".to_string() {
            result.push(BTreeSet::<char>::new());
        }
        else {
            for c in line.chars() {
                result.last_mut().unwrap().insert(c);
            }
        }
    }
    result
}

fn part1(filename: &str) -> usize {
    let ip = parse_input1(filename);
    let mut sum = 0;
    for s in ip {
        sum += s.len();
    }
    println!("aoc 2020 day 6 part 1 file {filename} answer = {sum}");
    sum
}

fn parse_input2(filename: &str) -> Vec<Vec<BTreeSet<char>>> {
    let mut result = vec![];
    result.push(vec![]);
    let lines = get_lines(filename);
    for line in lines {
        if line == "".to_string() {
            result.push(vec![]);
        }
        else {
            let mut v = BTreeSet::new();
            for c in line.chars() {
                v.insert(c);
            }
            result.last_mut().unwrap().push(v);
        }
    }
    result
}

fn both(set1: &BTreeSet<char>, set2: &BTreeSet<char>) -> BTreeSet<char> {
    let mut result = BTreeSet::new();
    for c in set1.intersection(&set2) {
        result.insert(*c);
    }
    result
}

fn inall(sets: &Vec<BTreeSet<char>>) -> BTreeSet<char> {
    let mut result = if let Some(set) = sets.first() {
        set.clone()
    }
    else {
        BTreeSet::new()
    };
    for i in 1..sets.len() {
        result = both(&result,&sets[i]);
    }
    result
}

fn part2(filename: &str) -> usize {
    let ip = parse_input2(filename);
    let mut sum = 0;
    for s in &ip {
        let set = inall(s);
        sum += set.len();
    }
    println!("aoc 2020 day 6 part 2 file {filename} answer = {sum}");
    sum
}

fn main() {
    part1("input");
    part1("test_input");
    part2("input");
    part2("test_input");

    //let all = (b'a' ..= b'z').map(char::from);
    //println!("{:?}",all);
    //for c in all {println!("{c}");}
}
