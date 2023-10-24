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

// Note this overly complicated solution
// was to help me learn const generics in rust
// I think a Vec<Vec<bool>> would have been a simpler approach
#[derive(Debug)]
struct TreeMap<const N: usize>(Vec<[bool; N]>);

impl<const N: usize> Default for TreeMap<N> {
    fn default() -> TreeMap<N> {
        TreeMap(vec![])
    }
}

impl<const N: usize> TreeMap<N> {
    fn at(&self, i: usize, j: usize) -> &bool {
        &self.0[i][j % N]
    }

    fn ht(&self) -> usize {
        self.0.len()
    }
}

fn parse_ip<const N: usize>(filename: &str) -> TreeMap<N> {
    let ip = get_lines(filename);
    let mut result = TreeMap::default();
    for line in ip {
        let mut row = [false; N];
        let mut iter = line.chars();
        for i in 0..N {
            if let Some(c) = iter.next() {
                if c == '#' {
                    row[i] = true;
                }
            }
        }
        result.0.push(row);
    }
    result
}

fn do_steps<const N: usize>(map: &TreeMap<N>, ni: usize, nj: usize) -> usize {
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    while i < map.ht() {
        if *map.at(i, j) {
            count += 1;
        }
        i += ni;
        j += nj;
    }
    count
}

fn do_part1<const N: usize>(filename: &str) -> usize {
    let ip = parse_ip::<N>(filename);
    let answer = do_steps::<N>(&ip, 1, 3);
    println!("aoc 2020 day 3 part 1 for file {filename} answer = {answer}");
    answer
}

fn do_part2<const N: usize>(filename: &str) -> usize {
    let ip = parse_ip::<N>(filename);
    let steps = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut answer = 1;
    for step in steps {
        answer *= do_steps::<N>(&ip, step.0, step.1);
    }
    println!("aoc 2020 day 3 part 2 for file {filename} answer = {answer}");
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(do_part1::<11>("test_input"), 7);
    }

    #[test]
    fn part2_test() {
        assert_eq!(do_part2::<11>("test_input"), 336);
    }
}

fn main() {
    println!("aoc 2020 day 1");

    do_part1::<11>("test_input");
    do_part1::<31>("input");
    do_part2::<11>("test_input");
    do_part2::<31>("input");
}
