use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
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

#[derive(Debug)]
enum Rule {
    Char(char),
    Subs(Vec<Vec<usize>>),
}

fn parse_input(filename: &str) -> (BTreeMap<usize, Rule>, Vec<String>) {
    let lines = get_lines(filename);
    let mut is_message = false;
    let mut rulemap = BTreeMap::new();
    let mut messages = vec![];

    for line in lines {
        if line == "" {
            is_message = true;
        } else if !is_message {
            let mut words = line.split(":");
            let key = words.next().unwrap().parse::<usize>().unwrap();
            let mut rules = String::from(words.next().unwrap());
            if rules.contains("\"") {
                rules = rules.trim().to_string();
                let c = rules.chars().nth(2).unwrap();
                rulemap.insert(key, Rule::Char(c));
            } else {
                let mut subs = vec![];
                for rule in rules.split("|") {
                    let mut v = vec![];
                    for s in rule.split_whitespace() {
                        v.push(s.parse::<usize>().unwrap());
                    }
                    subs.push(v);
                }
                rulemap.insert(key, Rule::Subs(subs));
            }
        } else {
            messages.push(line);
        }
    }
    (rulemap, messages)
}

fn get_combs(
    rules: &BTreeMap<usize, Rule>,
    cache: &mut BTreeMap<usize, Vec<String>>,
    i: usize,
    level: usize,
) -> Vec<String> {
    if level > 6 {
        return vec![];
    }
    if let Some(v) = cache.get(&i) {
        return v.clone();
    }
    match rules.get(&i).unwrap() {
        Rule::Char(c) => vec![format!("{c}")],
        Rule::Subs(v) => {
            let mut combs = vec![];
            for set in v {
                let mut combset = vec!["".to_string()];
                for rule in set {
                    let mut combsrule = vec![];
                    for combsi in get_combs(rules, cache, *rule, level + 1) {
                        for s in combset.iter_mut() {
                            combsrule.push(s.clone() + &combsi);
                        }
                    }
                    //println!{"{:?}",combsrule};
                    combset = combsrule;
                }
                combs.append(&mut combset);
            }
            cache.insert(i, combs.clone());
            combs
        }
    }
}

fn part1(filename: &str) {
    let (rules, messages) = parse_input(filename);
    let mut cache: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    //println!("{:?}",rules);
    let combs = get_combs(&rules, &mut cache, 0, 0);
    let combs: BTreeSet<_> = combs.iter().collect();
    //println!("{:?}",combs);
    let mut ans = 0;
    for message in &messages {
        if combs.contains(message) {
            ans += 1;
        }
    }
    println!("aoc202 day 19 part 1 file {filename}, answer = {ans}")
}

fn part2(filename: &str) {
    let (mut rules, messages) = parse_input(filename);
    *rules.get_mut(&8).unwrap() = Rule::Subs(vec![vec![42], vec![42, 8]]);
    *rules.get_mut(&11).unwrap() = Rule::Subs(vec![vec![42, 31], vec![42, 11, 31]]);
    let mut cache: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    //println!("{:?}",rules);
    let combs = get_combs(&rules, &mut cache, 0, 0);
    let combs: BTreeSet<_> = combs.iter().collect();
    //println!("{:?}",combs);
    let mut ans = 0;
    for message in &messages {
        if combs.contains(message) {
            ans += 1;
        }
    }
    println!("aoc202 day 19 part 2 file {filename}, answer = {ans}")
}

fn main() {
    part1("test_input");
    //part1("input");
    part1("test_input2");
    part2("test_input2");
}
