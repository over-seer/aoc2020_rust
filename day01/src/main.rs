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

fn parse_ip(filename: &str) -> BTreeSet<i64> {
    let ip = get_lines(filename);
    let mut result = BTreeSet::new();
    for line in ip {
        result.insert(line.parse::<i64>().unwrap());
        //if let Ok(i) = line.parse::<i64>() {
        //    result.push(i);
        //}
    }
    result
}

fn get_match(set: &BTreeSet<i64>, sum: i64) -> Option<i64> {
    for &i in set {
        //println!("{i}");
        let j = sum - i;
        if i != j {
            if set.contains(&j) {
                return Some(i);
            }
        }
    }
    None
}

fn get_match3(set: &BTreeSet<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    for &i in set {
        for &j in set {
            let k = sum - i - j;
            if k > 0 && k != i && k != j && i != j {
                if set.contains(&k) {
                    return Some((i, j, k));
                }
            }
        }
    }
    None
}

fn do_stuff(filename: &str) -> Option<i64> {
    println!("get match for file: {filename}");
    let ip = parse_ip(filename);
    if let Some(i) = get_match(&ip, 2020) {
        let j = 2020 - i;
        let product = i * j;
        println!("Answer for {filename} = {i}x{j}={product}");
        return Some(product);
    }
    None
}

fn do_stuff_part2(filename: &str) -> Option<i64> {
    println!("part 2 get match for file: {filename}");
    let ip = parse_ip(filename);
    if let Some((i, j, k)) = get_match3(&ip, 2020) {
        let product = i * j * k;
        println!("Answer for {filename} = {i}x{j}x{k}={product}");
        return Some(product);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = do_stuff("test_input");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 514579);
    }

    #[test]
    fn it_works2() {
        let result = do_stuff_part2("test_input");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 241861950);
    }
}

fn main() {
    println!("aoc 2020 day 1");
    do_stuff("input");
    do_stuff_part2("input");
    do_stuff("test_input");
    do_stuff_part2("test_input");
}
