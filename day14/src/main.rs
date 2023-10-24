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

fn parse_input(filename: &str) -> Vec<([char; 36], Vec<(u64, String)>)> {
    let mut result = vec![];
    let lines = get_lines(filename);
    let mut mask = ['X'; 36];
    for line in lines {
        let mut iter = line.split_whitespace();
        let first = iter.next().unwrap();
        if first == "mask" {
            let mut smask = iter.last().unwrap().chars();
            for i in 0..36 {
                mask[i] = smask.next().unwrap();
            }
            result.push((mask.clone(), vec![]));
        } else {
            let sm = first.split(['[', ']']).nth(1).unwrap();
            let m: u64 = sm.parse().unwrap();
            let b = iter.last().unwrap().to_string();
            result.last_mut().unwrap().1.push((m, b));
        }
    }
    result
}

fn apply_mask(mask: &[char; 36], snum: &String) -> u64 {
    //println!("{snum}");
    let num: u64 = snum.parse().unwrap();
    let snum = format!("{num:b}");
    let mut num = ['0'; 36];
    let mut v = vec![];
    for c in snum.chars() {
        v.push(c);
    }
    for i in 0..v.len() {
        num[36 - v.len() + i] = v[i];
    }
    //println!("{:?}",num);
    for i in 0..36 {
        if mask[i] == '1' {
            num[i] = '1';
        } else if mask[i] == '0' {
            num[i] = '0';
        }
    }

    let s: String = num.into_iter().collect();
    //println!("{s}");
    u64::from_str_radix(&s, 2).unwrap()
}

fn apply_mask_v2(mask: &[char; 36], addr: usize) -> Vec<u64> {
    //println!("{snum}");
    let mut result = vec![];
    let snum = format!("{addr:036b}");
    println!("{snum}");
    let mut num = ['0'; 36];
    let mut iter = snum.chars();
    for i in 0..36 {
        num[i] = iter.next().unwrap();
    }
    //println!("{:?}",num);
    let mut nx: u64 = 0;
    for i in 0..36 {
        if mask[i] == '1' {
            num[i] = '1';
        } else if mask[i] == 'X' {
            num[i] = 'X';
            nx += 1;
        }
    }

    println!("{:?}", num);

    for i in 0..(2 as u64).pow(nx as u32) {
        let mut num = num.clone();
        let sb = format!("{i:036b}");
        let mut iter = sb.chars();
        iter.nth((36 - nx - 1) as usize);
        println!("{nx} {:?}", iter);
        for c in num.iter_mut() {
            if *c == 'X' {
                *c = iter.next().unwrap();
            }
        }
        let s: String = num.into_iter().collect();
        println!("{s}");
        result.push(u64::from_str_radix(&s, 2).unwrap());
    }
    result
}

fn update_memmap(mask: &[char; 36], memip: &Vec<(u64, String)>, memmap: &mut BTreeMap<u64, u64>) {
    for (loc, val) in memip {
        let val = apply_mask(mask, val);
        memmap.entry(*loc).and_modify(|v| *v = val).or_insert(val);
    }
}

fn update_memmap_v2(
    mask: &[char; 36],
    memip: &Vec<(u64, String)>,
    memmap: &mut BTreeMap<u64, u64>,
) {
    for (loc, val) in memip {
        let locs = apply_mask_v2(mask, *loc as usize);
        let val = val.parse::<u64>().unwrap();
        for loc in locs {
            memmap.entry(loc).and_modify(|v| *v = val).or_insert(val);
        }
    }
}

fn part1(filename: &str) {
    let ip = parse_input(filename);
    let mut memmap = BTreeMap::new();

    let mut sum = 0;
    for (mask, memip) in ip {
        println!("{:?}", mask);
        update_memmap(&mask, &memip, &mut memmap);
    }
    for (key, val) in &memmap {
        println!("{key} {val}");
    }
    sum += memmap.values().sum::<u64>();
    println!("aoc 2020 part 1 file {filename}, answer = {sum}");
}

fn part2(filename: &str) {
    let ip = parse_input(filename);
    let mut memmap = BTreeMap::new();

    for (mask, memip) in ip {
        println!("{:?}", mask);
        update_memmap_v2(&mask, &memip, &mut memmap);
    }
    for (key, val) in &memmap {
        println!("{key} {:?}", val);
    }
    let sum = memmap.values().sum::<u64>();
    println!("aoc 2020 part 2 file {filename}, answer = {sum}");
}

fn main() {
    part1("test_input");
    part1("input");
    part2("test_input2");
    part2("input");
}
