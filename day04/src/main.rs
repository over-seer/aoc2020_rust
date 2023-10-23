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

fn parse_input(filename: &str) -> Vec<BTreeMap<String,String>> {
    let mut result = vec![];
    result.push(BTreeMap::new());
    let lines = get_lines(filename);
    for line in lines {
        //println!("{line}");
        if line == "".to_string() {
            result.push(BTreeMap::<String,String>::new());
        }
        else {
            for pr in line.split_whitespace() {
                let mut iter = pr.split(":");
                let key = iter.next().unwrap().to_string();
                let value = iter.next().unwrap().to_string();
                result.last_mut().unwrap().insert(key, value);
            }
        }
    }
    result
}

fn is_valid_passport_part1(pp: &BTreeMap<String,String>) -> bool {
    let req = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    let mut count = 0;
    for &key in &req {
        if let Some(_val) = pp.get(key) {
            count += 1;
        }
    }
    count >= req.len()
}

fn do_part1(filename: &str) -> usize {
    let ip = parse_input(filename);
    let mut total = 0;
    for passport in ip {
        if is_valid_passport_part1(&passport) { total += 1; }
    }
    println!("aoc 2020 day 4 part 1 for file {filename}, answer = {total}");
    total
}

fn is_valid_num(s: &str, low: usize, high: usize) -> bool {
    if let Ok(n) = s.parse::<usize>() {
        return n >= low && n <= high;
    }
    false
}

/* 
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.
*/
fn is_valid_field(field: &str, content: &str) -> bool {
    if field == "byr" {
        return is_valid_num(content,1920,2002);
    } else if field == "iyr" {
        return is_valid_num(content,2010,2020);
    }  else if field == "eyr" {
        return is_valid_num(content,2020,2030);
    } else if field == "hgt" {
        if content.ends_with("cm") {
            return is_valid_num(content.split("cm").next().unwrap(),150,193);
        } else if content.ends_with("in") {
            return is_valid_num(content.split("in").next().unwrap(),59,76);
        }
        return false;
    } else if field == "hcl" {
        let mut iter = content.chars();
        if content.len() == 7 && Some('#') == iter.next() {
            for c in iter {
                if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                    return false;
                }
            }
            return true;
        }
        return false;
    } else if field == "ecl" {
        return BTreeSet::from(["amb","blu","brn","gry","grn","hzl","oth"]).contains(content);
        //return (content == "amb") || (content == "blu") || (content == "brn") || (content == "gry") || (content == "grn") || (content == "hzl") || (content == "oth");
    } else if field == "pid" {
        return content.len() == 9;
    } else if field == "cid" { return true; }
    false
}

fn is_valid_passport_part2(pp: &BTreeMap<String,String>) -> bool {
    let req = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    let mut count = 0;
    for &key in &req {
        if let Some(val) = pp.get(key) {
            if is_valid_field(key,val) {
                count += 1;
            }
        }
    }
    count >= req.len()
}

fn do_part2(filename: &str) -> usize {
    let ip = parse_input(filename);
    //println!("{:?}",ip);
    let mut total = 0;
    for passport in ip {
        if is_valid_passport_part2(&passport) {
            total += 1;
        }
    }
    println!("aoc 2020 day 4 part 2 for file {filename}, answer = {total}");
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test_valid() {
        assert_eq!(do_part2("all_valid"),4);
    }

    #[test]
    fn part2_test_valid2() {
        let ip = parse_input("all_valid");
        for passport in ip {
            for (key,val) in &passport {
                println!("{key} : {val}");
                assert!(is_valid_field(&key,&val));
            }
        }
    }
    
    #[test]
    fn part2_test_invalid() {
        assert_eq!(do_part2("all_invalid"),0);
    }

}


fn main() {
    do_part1("input");
    do_part1("test_input");
    do_part2("input");
    do_part2("test_input");

}

