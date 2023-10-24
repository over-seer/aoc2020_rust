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

fn get_lines(filename: &str) -> Vec<usize> {
    let mut result = vec![];
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip.parse::<usize>().unwrap());
            }
        }
    }
    result
}

fn is_sum(val: usize, nos: &[usize]) -> bool {
    for i in 0..nos.len() {
        for j in i + 1..nos.len() {
            if nos[i] + nos[j] == val {
                return true;
            }
        }
    }
    false
}

fn part1(filename: &str, n: usize) -> usize {
    let ip = get_lines(filename);
    for i in n..ip.len() {
        let val = ip[i];
        let preceding = &ip[i - n..i];
        //println!("{:?}",preceding);
        if !is_sum(val, preceding) {
            println!("aoc 2020 day 9 part 1 for file {filename}, answer = {val}");
            return val;
        }
    }
    0
}

fn get_sum(target: usize, nos: &[usize]) -> Option<&[usize]> {
    let mut sum = 0;
    for i in 0..nos.len() {
        let n = nos[i];
        sum += n;
        if sum == target {
            return Some(&nos[0..i]);
        } else if sum > target {
            return None;
        }
    }
    None
}

fn part2(filename: &str, target: usize) -> usize {
    let ip = get_lines(filename);
    for i in 0..ip.len() {
        if let Some(nos) = get_sum(target, &ip[i..ip.len()]) {
            //get min and max in range
            let nmin = nos.iter().min().unwrap();
            let nmax = nos.iter().max().unwrap();
            let answer = nmin + nmax;
            println!("aoc 2020 day 9 part 2 for file {filename}, answer = {answer}");
            return answer;
        }
    }
    0
}

fn main() {
    let test_target = part1("test_input", 5);
    let target = part1("input", 25);

    part2("test_input", test_target);
    part2("input", target);
}
