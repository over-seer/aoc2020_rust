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

fn parse_input(filename: &str) -> Vec<(String, i64)> {
    let mut result = vec![];
    for line in get_lines(filename) {
        let mut iter = line.split_whitespace();
        result.push((
            iter.next().unwrap().to_string(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        ));
    }
    result
}

fn do_instruction(
    mut accum: i64,
    instructions: &Vec<(String, i64)>,
    mut pos: usize,
    mut checklist: Vec<bool>,
) -> (i64, bool) {
    if pos == instructions.len() {
        return (accum, true);
    } else if checklist[pos] {
        return (accum, false);
    } else {
        checklist[pos] = true;
        let (op, arg) = &instructions[pos];

        if op == "nop" {
            pos += 1;
        } else if op == "acc" {
            accum += arg;
            pos += 1;
        } else if op == "jmp" {
            pos = (pos as i64 + arg) as usize;
        }
        return do_instruction(accum, instructions, pos, checklist);
    }
}

fn part1(filename: &str) -> i64 {
    let instructions = parse_input(filename);
    let checklist = vec![false; instructions.len()];
    let (answer, _term) = do_instruction(0, &instructions, 0, checklist);
    println!("aoc 2020 part 1 file {filename}, answer = {answer}");
    answer
}

fn part2(filename: &str) -> Option<i64> {
    let instructions = parse_input(filename);
    for i in 0..instructions.len() {
        let (op, arg) = &instructions[i];
        if op == "jmp" || op == "nop" {
            let checklist = vec![false; instructions.len()];
            let mut instructions = instructions.clone();
            instructions[i] = if op == "jmp" {
                ("nop".to_string(), *arg)
            } else {
                ("jmp".to_string(), *arg)
            };
            let (accum, did_terminate) = do_instruction(0, &instructions, 0, checklist);
            if did_terminate {
                println!("aoc 2020 day 8 part 2 file {filename}, answer = {accum}");
                return Some(accum);
            }
        }
    }
    None
}

fn main() {
    part1("test_input");
    part1("input");
    part2("test_input");
    part2("input");
}
