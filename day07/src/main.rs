use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

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

fn get_words(line: &str) -> Vec<String> {
    let mut result = vec![];
    for s in line.split_whitespace() {
        result.push(s.to_string());
    }
    result
}

fn parse_input(filename: &str) -> BTreeMap<(String,String),BTreeMap<(String,String),usize>> {
    let mut result = BTreeMap::new();
    let lines = get_lines(filename);
    for line in lines {
        let mut rule = BTreeMap::new();
        let words = get_words(&line);
        let bag = (words[0].clone(),words[1].clone());
        if words.len() % 4 == 0 {
            // /let ntypes = (words.len() - 4) / 4;
            for i in (4..words.len()).step_by(4) {
                let n : usize = words[i].parse().unwrap();
                let bag_type = (words[i+1].clone(),words[i+2].clone());
                rule.insert(bag_type,n);
            }
        } 
        result.insert(bag,rule);
    }
    result
}

fn does_contain(
    inner: &(String,String),
    outer: &(String,String),
    rules: &BTreeMap<(String,String),BTreeMap<(String,String),usize>>
) -> bool {
    if let Some(rule) = rules.get(outer) {
        for (bag,_n) in rule {
            if bag == inner {
                println!("bag {:?} contains {:?}",outer, inner);
                return true;
            } else {
                if does_contain(inner,bag,rules) {
                    return true
                }
            }
        }
    }
    false
}

fn part1(filename: &str) -> usize {
    let mut answer = 0;
    let rules = parse_input(filename);
    let inner = ("shiny".to_string(),"gold".to_string());
    for (outer,_rule) in &rules {
        if does_contain(&inner, &outer, &rules) {
            println!("{:?} TRUE",outer);
            answer += 1;
        }
        println!("{:?} FALSE",outer);
    }
    println!("aoc 2020 day 7 part 1 for file {filename}, answer = {answer}");
    answer
}

fn contains_n(
    outer: &(String,String),
    rules: &BTreeMap<(String,String),BTreeMap<(String,String),usize>>
) -> usize {
    let mut nbags = 0;
    if let Some(rule) = rules.get(outer) {
        for (bag,n) in rule {
            nbags += n + n * contains_n(bag,rules);
        }
    }
    nbags
}


fn part2(filename: &str) -> usize {
    let rules = parse_input(filename);
    let nbags = contains_n(&("shiny".to_string(),"gold".to_string()), &rules);
    println!("aoc 2020 day 7 part 2 for file {filename}, answer = {nbags}");
    nbags
}

fn main() {
    part1("test_input");
    part1("input");
    part2("test_input");
    part2("test_input2");
    part2("input");
}
