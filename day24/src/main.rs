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

fn parse_instruction(mut line: String) -> Vec<[i32; 2]> {
    let mut result = vec![];
    while !line.is_empty() {
        //println!("{line}");
        if line.starts_with("ne") {
            line = line.strip_prefix("ne").unwrap().to_string();
            result.push([1, -1]);
        } else if line.starts_with("nw") {
            line = line.strip_prefix("nw").unwrap().to_string();
            result.push([0, -1]);
        } else if line.starts_with("se") {
            line = line.strip_prefix("se").unwrap().to_string();
            result.push([0, 1]);
        } else if line.starts_with("sw") {
            line = line.strip_prefix("sw").unwrap().to_string();
            result.push([-1, 1]);
        } else if line.starts_with("e") {
            line = line.strip_prefix("e").unwrap().to_string();
            result.push([1, 0]);
        } else if line.starts_with("w") {
            line = line.strip_prefix("w").unwrap().to_string();
            result.push([-1, 0]);
        } else {
            println!("ERROR");
        }
    }
    result
}

fn parse(filename: &str) -> Vec<Vec<[i32; 2]>> {
    let lines = get_lines(filename);
    let mut result = vec![];
    for line in lines {
        result.push(parse_instruction(line.clone()));
    }
    result
}

fn addij(ij: &mut [i32; 2], didj: &[i32; 2]) {
    ij[0] += didj[0];
    ij[1] += didj[1];
}

fn part1(filename: &str) -> BTreeSet<[i32; 2]> {
    let ip = parse(filename);
    let mut blacks = BTreeSet::new();
    for line in ip {
        let mut ij = [0, 0];
        for didj in line {
            addij(&mut ij, &didj);
        }
        if blacks.contains(&ij) {
            blacks.remove(&ij);
        } else {
            blacks.insert(ij);
        }
    }
    let ans = blacks.len();
    println!("aoc 2020 day 24 part 1 file {filename} ans = {ans}");
    blacks
}

fn neighbours(ij: &[i32; 2]) -> [[i32; 2]; 6] {
    let mut nbrs = [[-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0]];
    for nbr in nbrs.iter_mut() {
        nbr[0] += ij[0];
        nbr[1] += ij[1];
    }
    return nbrs;
}

fn still_black(blacks: &BTreeSet<[i32; 2]>, ij0: &[i32; 2]) -> bool {
    let mut count = 0;
    for ij in neighbours(ij0) {
        if blacks.contains(&ij) {
            count += 1;
        }
        if count > 2 {
            break;
        }
    }
    !(count == 0 || count > 2)
}

fn becomes_black(
    blacks: &BTreeSet<[i32; 2]>,
    known: &[i32; 2],
    ij0: &[i32; 2],
) -> bool {
    let mut count = 1;
    for ij in neighbours(ij0) {
        if ij != *known {
            if blacks.contains(&ij) {
                count += 1;
            }
            if count > 2 {
                break;
            }
        }
    }
    count == 2
}


fn process(old_blacks: &BTreeSet<[i32; 2]>) -> BTreeSet<[i32; 2]> {
    let mut new_blacks = BTreeSet::new();
    for ij in old_blacks {
        if still_black(&old_blacks, ij) {
            new_blacks.insert(*ij);
        }
        for ij_prime in neighbours(ij) {
            if !old_blacks.contains(&ij_prime) {
                if becomes_black(old_blacks,ij,&ij_prime) {
                    new_blacks.insert(ij_prime);
                }
            }
        }
    }
    new_blacks
}

fn part2(filename: &str) {
    let mut blacks = part1(filename);
    for _i in 0..100 {
        blacks = process(&blacks);
    }

    let ans = blacks.len();
    println!("aoc 2020 day 24 part 2 file {filename} ans = {ans}");
}

fn main() {
    part2("test_input");
    part2("input");
}
