use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn parse_input(filename: &str) -> (u64,Vec<String>) {
    let ip = get_lines(filename);
    let mut nos = vec![];
    for s in ip[1].split(",") {
        nos.push(s.to_string());
    }
    (ip[0].parse().unwrap(),nos)
}

fn parse_part1(filename: &str) -> (u64,Vec<u64>) {
    let (t,ip) = parse_input(filename);
    let mut nos = vec![];
    for s in ip {
        if s != "x" {
            nos.push(s.parse().unwrap());
        }
    }
    (t,nos)
}

fn next_bus(t: u64, f: u64) -> u64{
    f - (t - 1) % f - 1
}

fn part1(filename: &str) -> u64{
    let (t,ids) = parse_part1(filename);
    println!("{t} {:?}",ids);
    let mut id_min_wait = ids[0];
    let mut min_wait = next_bus(t,id_min_wait);
    for i in ids {
        let iwait = next_bus(t,i);
        //println!("{i} {iwait}");
        if iwait < min_wait {
            id_min_wait = i;
            min_wait = iwait;
        } 
    }
    let ans = id_min_wait * min_wait;
    println!("{id_min_wait}, {min_wait}");
    println!("aoc 2020 day 13 part 1 file {filename}, answer = {ans}");
    
    ans
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
}


fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a,b)
}


fn next_match(mut current : u64, mut offset: u64, step: u64, v2: u64) -> (u64,u64) {
    loop {
        while v2 <= offset {
            offset -= v2;
        }
        if (v2 - current % v2) ==  offset {
            return (current, lcm(step,v2));
        }
        current += step
    }
}

fn part2(filename: &str) -> u64 {
    let (_t,snos) = parse_input(filename);
    let mut nos = vec![];
    for s in snos {
        nos.push({
            if s == "x" { 0 as u64 }
            else { s.parse().unwrap() }
        });
    }
    let mut step = nos[0];
    let mut current = 0;
    for i in 1..nos.len() {
        let next = nos[i];
        if next != 0 {
            print!("{current} {step} {next} {i} ->  ");
            (current,step) = next_match(current,i as u64,step,next);
            println!("{current} {step}");
        }
    }
    let ans = current;
    println!("aoc 2020 day 13 part 2 file {filename}, answer = {ans}");
    ans
}

#[cfg(test)]
mod tests {
    use crate::next_bus;
    #[test]
    fn next_bus_test() {
        assert_eq!(next_bus(200, 10),0);
        assert_eq!(next_bus(201, 10),9);
        assert_eq!(next_bus(209, 10),1);
    }
}

fn main() {
    part1("test_input");
    part1("input");
    part2("test_input");
    part2("input");
}
