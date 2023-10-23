use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
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

fn parse_range(s: &str) -> Range<usize> {
    let a: usize = s.split('-').next().unwrap().trim().parse().unwrap();
    let b: usize = s.split('-').last().unwrap().trim().parse().unwrap();
    a..b + 1
}

fn parse_input(
    filename: &str,
) -> (
    BTreeMap<String, (Range<usize>, Range<usize>)>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let lines = get_lines(filename);
    let mut map = BTreeMap::new();
    let mut mine = vec![];
    let mut nearby = vec![];
    let mut is_mine = true;
    let mut is_ticket = false;
    for line in lines {
        if line.trim() == "" || line == "your ticket:" || line == "nearby tickets:" {
            is_ticket = true;
        } else if is_ticket {
            let mut v = vec![];
            for n in line.split(',') {
                v.push(n.trim().parse::<usize>().unwrap());
            }
            if is_mine {
                mine = v;
                is_mine = false;
            } else {
                nearby.push(v);
            }
        } else {
            let mut keyval = line.split(':');
            let key = keyval.next().unwrap().trim();
            //println!("{:?}",keyval);
            let mut words = keyval.next().unwrap().split(" or ");
            let range1 = parse_range(words.next().unwrap());
            let range2 = parse_range(words.last().unwrap());
            map.insert(key.to_string(), (range1, range2));
        }
    }
    (map, mine, nearby)
}

fn could_be(map: &BTreeMap<String, (Range<usize>, Range<usize>)>, i: usize) -> bool {
    for (r1, r2) in map.values() {
        if r1.contains(&i) || r2.contains(&i) {
            return true;
        }
    }
    false
}

fn part1(filename: &str) {
    let (fields, _mine, nearby) = parse_input(filename);
    //println!("{:?}", parse_input(filename));
    let mut count = 0;
    for i in nearby.iter().flatten() {
        //println!("{i}");
        if !could_be(&fields, *i) {
            count += i;
        }
    }
    println!("aoc 2020 day 16 part 1 file {filename} answer = {count}")
}

fn get_part2_ip(
    filename: &str,
) -> (
    BTreeMap<String, (Range<usize>, Range<usize>)>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let (fields, mine, nearby) = parse_input(filename);
    let mut good_tickets: Vec<Vec<usize>> = vec![];
    for ticket in &nearby {
        let mut is_good = true;
        for i in ticket {
            if !could_be(&fields, *i) {
                is_good = false;
                break;
            }
        }
        if is_good {
            good_tickets.push(ticket.clone());
        }
    }
    (fields, mine, good_tickets)
}

fn set_field(
    checklist: &mut Vec<Vec<BTreeSet<String>>>,
    keys: &mut BTreeSet<String>,
    fields: &mut BTreeMap<String, usize>,
    i: usize,
    key: &String
) {
    fields.insert(key.clone(), i);
    keys.remove(key);
    for ticket in checklist {
        for ifield in 0..ticket.len() {
            if ifield != i {
                ticket[ifield].remove(key);
            }
        }
    }
}

fn narrow(
    checklist: &mut Vec<Vec<BTreeSet<String>>>,
    keys: &mut BTreeSet<String>,
    fields: &mut BTreeMap<String, usize>,
) {
    let ntickets = checklist.len();
    let nfields = checklist[0].len();

    for ifield in 0..nfields {
        let mut set = keys.clone();
        for iticket in 0..ntickets {
            for key in keys.iter() {
                if !checklist[iticket][ifield].contains(key) {
                    set.remove(key);
                }
            }
        }
        //println!("{:} {:?}", ifield, set);
        if set.len() == 1 {
            set_field(checklist, keys, fields, ifield, set.first().unwrap());
        }
    }
}

fn part2(filename: &str) {
    let (ranges, mine, nearby) = get_part2_ip(filename);

    let mut all = vec![];
    all.push(mine.clone());
    for t in &nearby {
        all.push(t.clone());
    }

    let mut map = BTreeMap::new();
    for key in ranges.keys() {
        map.insert(key.clone(), true);
    }

    let ntickets = all.len();
    let nfields = mine.len();
    let ntypes = ranges.len();
    let mut keys = BTreeSet::new();
    let mut fields: BTreeMap<String, usize> = BTreeMap::new();
    for key in ranges.keys() {
        keys.insert(key.clone());
    }

    let mut check = vec![vec![keys.clone(); nfields]; ntickets];

    println!("num tickets {ntickets} num fields {nfields} num types {ntypes}");

    for iticket in 0..ntickets {
        for ifield in 0..nfields {
            for (key, (r1, r2)) in &ranges {
                let val = all[iticket][ifield];
                if !r1.contains(&val) && !r2.contains(&val) {
                    check[iticket][ifield].remove(key);
                }
            }
        }
    }

    while keys.len() > 0 {
        narrow(&mut check, &mut keys, &mut fields);
    }
    //println!("{:?}", fields);

    let mut ans = 1;
    for (key,index) in &fields {
        if key.starts_with("departure") {
            ans *= mine[*index];
        }
    }
    println!("aoc 2020 day 16 part 2 file {filename} answer = {ans}")

}

fn main() {
    part1("test_input");
    part1("input");

    part2("test_input2");
    part2("input");
}
