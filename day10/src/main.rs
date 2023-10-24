use std::cmp::min;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
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

fn get_lines(filename: &str) -> BTreeSet<usize> {
    let mut result = BTreeSet::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                result.insert(ip.parse::<usize>().unwrap());
            }
        }
    }
    result.insert(result.last().unwrap() + 3);
    result
}

fn part1(filename: &str) -> usize {
    let ip = get_lines(filename);
    let mut dist = BTreeMap::from([(1_usize, 0_usize), (2_usize, 0_usize), (3_usize, 0_usize)]);
    let mut last = 0_usize;
    for i in ip {
        let diff = i - last;
        *dist.get_mut(&diff).unwrap() += 1;
        last = i;
    }
    let rating = dist.get(&1).unwrap() * dist.get(&3).unwrap();
    println!("aoc 2020 day 10 part 1 for file {filename}, answer = {rating}");
    rating
}

fn path_count(paths: &mut BTreeMap<usize, usize>) -> usize {
    let mut prev = VecDeque::new();
    for (i, n) in paths.iter_mut() {
        for (iprev, nprev) in prev.iter() {
            let lim = *i - min(*i, 3);
            let ip: usize = *iprev;
            if ip >= lim {
                let nprev: usize = *nprev;
                *n += nprev;
            }
        }
        prev.push_back((*i, *n));
        if prev.len() > 3 {
            prev.pop_front();
        }
    }
    let (_j, n) = paths.last_key_value().unwrap();
    *n
}

fn part2(filename: &str) -> usize {
    let ip = get_lines(filename);
    let mut paths = BTreeMap::new();
    for i in ip {
        paths.insert(i, 0_usize);
    }
    paths.insert(0, 1);

    let answer = path_count(&mut paths);
    println!("aoc 2020 day 10 part 2 file {filename}, answer = {answer}");
    answer
}

fn main() {
    part1("test_input");
    part1("input");
    part2("test_input");
    part2("input");
}
