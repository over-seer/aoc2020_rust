use std::cmp::max;
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

fn get_seat(s: &str) -> (usize, usize) {
    let mut s1 = String::from("");
    let mut s2 = s1.clone();
    for c in s.chars() {
        if c == 'F' {
            s1.push('0');
        } else if c == 'B' {
            s1.push('1');
        } else if c == 'R' {
            s2.push('1');
        } else if c == 'L' {
            s2.push('0');
        }
    }
    //println!("{s1} {s2}");
    (
        usize::from_str_radix(&s1, 2).unwrap(),
        usize::from_str_radix(&s2, 2).unwrap(),
    )
}

fn parse_input(filename: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let lines = get_lines(filename);
    for line in lines {
        result.push(get_seat(&line));
    }
    result
}

fn both_parts(filename: &str) -> (usize, usize) {
    let ip = parse_input(filename);
    let mut set: BTreeSet<usize> = BTreeSet::new();
    let mut max_id = 0;
    for (row, col) in ip {
        let id = row * 8 + col;
        max_id = max(max_id, id);
        set.insert(id);
    }

    let mut my_id = 0;
    for i in 1..max_id {
        if !set.contains(&i) {
            if set.contains(&(i - 1)) && set.contains(&(i + 1)) {
                my_id = i;
            }
        }
    }

    println!("aoc 2020 day 5 for file {filename}, max id = {max_id}, my id = {my_id}");
    (max_id, my_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_seat_test() {
        assert_eq!(get_seat("BFFFBBFRRR"), (70, 7));
        assert_eq!(get_seat("FFFBBBFRRR"), (14, 7));
        assert_eq!(get_seat("BBFFBBFRLL"), (102, 4));
    }
}

fn main() {
    let ip = parse_input("input");
    println!("{:?}", ip);
    println!("Hello, world!");
    both_parts("input");
}
