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
struct Policy {
    pub min: usize,
    pub max: usize,
    pub c: char,
}

fn parse_ip(filename: &str) -> Vec<(Policy, String)> {
    let ip = get_lines(filename);
    let mut result = vec![];
    for line in ip {
        let mut iter = line.split_whitespace();
        let minmax = iter.next().unwrap();
        let c = iter.next().unwrap();
        let pw = iter.next().unwrap();

        let mut iter = minmax.split("-");
        let min: usize = iter.next().unwrap().parse().unwrap();
        let max: usize = iter.next().unwrap().parse().unwrap();

        let c = c.chars().next().unwrap();

        result.push((Policy { min, max, c }, pw.to_string()));
    }
    result
}

fn meets_policy1(policy: &Policy, pw: &str) -> bool {
    let mut count = 0;
    for c in pw.chars() {
        if c == policy.c {
            count += 1;
        }
    }
    count >= policy.min && count <= policy.max
}

fn meets_policy2(policy: &Policy, pw: &str) -> bool {
    let mut count = 0;
    if let Some(c) = pw.chars().nth(policy.min - 1) {
        if c == policy.c {
            count += 1;
        }
    }

    if let Some(c) = pw.chars().nth(policy.max - 1) {
        if c == policy.c {
            count += 1;
        }
    }
    count == 1
}

fn do_part1(filename: &str) -> i64 {
    let ip = parse_ip(filename);
    let mut count = 0;
    for line in ip {
        if meets_policy1(&line.0, &line.1) {
            count += 1;
        }
    }
    println!("aoc 2020 day 2 part 1 answer = {count}");
    count
}

fn do_part2(filename: &str) -> i64 {
    let ip = parse_ip(filename);
    let mut count = 0;
    for line in ip {
        if meets_policy2(&line.0, &line.1) {
            count += 1;
        }
    }
    println!("aoc 2020 day 2 part 2 for file {filename}, answer = {count}");
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meets_policy1_test() {
        assert!(true);
        let ip = parse_ip("test_input");
        assert!(meets_policy1(&ip[0].0, &ip[0].1));
        assert!(!meets_policy1(&ip[1].0, &ip[1].1));
        assert!(meets_policy1(&ip[2].0, &ip[2].1));
        assert!(meets_policy1(
            &Policy {
                min: 2,
                max: 2,
                c: 'c'
            },
            "cc"
        ));
        assert!(meets_policy1(
            &Policy {
                min: 8,
                max: 14,
                c: 'm'
            },
            "mmmmmmsmmmmmm"
        ));
    }

    #[test]
    fn part1_test() {
        assert_eq!(do_part1("test_input"), 2);
    }

    #[test]
    fn meets_policy2_test() {
        assert!(true);
        let ip = parse_ip("test_input");
        assert!(meets_policy2(&ip[0].0, &ip[0].1));
        assert!(!meets_policy2(&ip[1].0, &ip[1].1));
        assert!(!meets_policy2(&ip[2].0, &ip[2].1));
        assert!(!meets_policy2(
            &Policy {
                min: 1,
                max: 2,
                c: 'c'
            },
            "cc"
        ));
        //assert!(meets_policy1(&Policy { min: 8, max: 14, c: 'm' }, "mmmmmmsmmmmmm"));
    }

    #[test]
    fn part2_test() {
        assert_eq!(do_part2("test_input"), 1);
    }
}

fn main() {
    println!("aoc 2020 day 1");
    let ip = parse_ip("test_input");
    for (pol, pw) in ip {
        println!("{:?} {pw}", pol);
    }
    do_part1("input");
    do_part2("input");
    do_part2("test_input");
    //do_stuff_part2("input");
    //do_stuff("test_input");
    //do_stuff_part2("test_input");
}
