use std::collections::BTreeMap;
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

fn eq(line: &str) -> usize {
    let mut tokens = line.split_whitespace();
    let mut a = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut op = '+';
    for token in tokens {
        if token == "+" || token == "*" {
            op = token.chars().nth(0).unwrap();
        } else if op == '*' {
            a *= token.parse::<usize>().unwrap();
        } else if op == '+' {
            a += token.parse::<usize>().unwrap();
        }
    }
    a
}

fn brackeq(line: &str, rule: i8) -> usize {
    let mut line_copy = String::from(line);
    let mut map = BTreeMap::new();
    for (i, s) in line.match_indices("(") {
        //println!("{i} {s}");
        map.insert(i, s.chars().nth(0).unwrap());
    }
    for (i, s) in line.match_indices(")") {
        //println!("{i} {s}");
        map.insert(i, s.chars().nth(0).unwrap());
    }

    let mut level = 0;
    let mut i0 = 0 as usize;
    for (i, c) in &map {
        if *c == '(' {
            level += 1;
            if level == 1 {
                i0 = *i;
            }
        } else if *c == ')' {
            if level == 1 {
                // do something
                let inside = &line[i0 + 1..*i];
                let pattern = &line[i0..*i + 1];
                //println!("inside {inside} outside {pattern}");
                let val = brackeq(inside, rule);
                line_copy = line_copy.replacen(pattern, &val.to_string(), 1);
            }
            level -= 1;
        }
    }
    if rule == 1 {
        eq(&line_copy)
    } else {
        eq2(&line_copy)
    }
}

fn part1(filename: &str) {
    let ip = get_lines(filename);
    let mut ans = 0;
    for line in &ip {
        ans += brackeq(line, 1);
    }
    println!("aoc 2020 day 18 part 1 answer = {ans}");
}

fn eq2(line: &str) -> usize {
    let mut tokens: Vec<String> = line.split_whitespace().map(String::from).collect();
    for i in 0..tokens.len() {
        if tokens[i] == "+" {
            let mut a = tokens[i - 1].parse::<usize>().unwrap();
            let b = tokens[i + 1].parse::<usize>().unwrap();
            a += b;
            tokens[i - 1] = "1".to_string();
            tokens[i + 1] = a.to_string();
            tokens[i] = "*".to_string();
        }
    }
    let new_line: String = tokens.join(" ");
    eq(&new_line)
}

fn part2(filename: &str) {
    let ip = get_lines(filename);
    let mut ans = 0;
    for line in &ip {
        ans += brackeq(line, 2);
    }
    println!("aoc 2020 day 18 part 2 answer = {ans}");
}

#[cfg(test)]
mod tests {
    use crate::brackeq;
    #[test]
    fn brackeq_test() {
        assert_eq!(brackeq("2 * 3 + (4 * 5)", 1), 26);
        assert_eq!(brackeq("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1), 437);
        assert_eq!(
            brackeq("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 1),
            12240
        );
        assert_eq!(
            brackeq("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 1),
            13632
        );
    }

    #[test]
    fn brackeq2_test() {
        assert_eq!(brackeq("2 * 3 + (4 * 5)", 2), 46);
        assert_eq!(brackeq("5 + (8 * 3 + 9 + 3 * 4 * 3)", 2), 1445);
        assert_eq!(
            brackeq("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 2),
            669060
        );
        assert_eq!(
            brackeq("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 2),
            23340
        );
    }
}

fn main() {
    part1("input");
    part2("input");
}
