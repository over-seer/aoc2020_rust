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

fn parse_input(filename: &str) -> Vec<(char,i32)> {
    let mut result = vec![];
    let lines = get_lines(filename);
    for line in lines {
        let mut iter = line.chars();
        let c = iter.next().unwrap();
        let n : i32 = iter.as_str().parse().unwrap();
        result.push((c,n));
    }
    result
}

fn r(dir: &Pt) -> Pt {
    Pt{n: -dir.e, e: dir.n}
}

fn l(dir: &Pt) -> Pt {
    Pt{n: dir.e, e: -dir.n}
}

#[derive(Debug, Clone)]
pub struct Pt {
    pub n: i32,
    pub e: i32,
}
#[derive(Debug, Clone)]
struct State { pos: Pt, dir: Pt }


impl State {
    fn step(& mut self, dir: &Pt, val: i32) {
        self.pos.n += dir.n * val as i32;
        self.pos.e += dir.e * val as i32;
    }

    fn go (&mut self, instruction: (char, i32)) {
        let (action, value) = instruction;
        let (this_dir,dist) = if action == 'R' {
            let n = value / 90;
            for _i in 0..n {
                self.dir = r(&self.dir);
            }
            (self.dir.clone(),0)
        } else if action == 'L' {
            let n = value / 90;
            for _i in 0..n {
                self.dir = l(&self.dir);
            }
            (self.dir.clone(),0)
        } else if action == 'N' {
            (Pt{n: 1, e:0},value)
        } else if action == 'E' {
            (Pt{n: 0, e:1},value)
        } else if action == 'S' {
            (Pt{n: -1, e:0},value)
        } else if action == 'W' {
            (Pt{n: 0, e:-1},value)
        } else {
            (self.dir.clone(),value)
        };
        self.step(&this_dir,dist);
        //println!("{:?}",self);
    }
}

fn part1(filename: &str) -> i32 {
    let mut state = State{pos: Pt{n: 0, e: 0}, dir: Pt{n: 0, e: 1}};
    let ip = parse_input(filename);
    for instruction in ip {
        state.go(instruction);
    }
    let md = state.pos.n.abs() + state.pos.e.abs();
    println!("{:?}",state);
    println!("aoc 2020 day 12 part 1 for file {filename}, answer = {md}");
    md
}

impl Pt {
    fn step(& mut self, dir: &Pt, val: i32) {
        self.n += dir.n * val as i32;
        self.e += dir.e * val as i32;
    }
}

fn update(instruction: (char, i32), waypoint: & mut Pt, ship: & mut Pt) {

    let (action, value) = instruction;
    if action == 'R' {
        let n = value / 90;
        for _i in 0..n {
            *waypoint = r(waypoint);
        }
    } else if action == 'L' {
        let n = value / 90;
        for _i in 0..n {
            *waypoint = l(waypoint);
        }
    } else if action == 'N' {
        waypoint.step(&Pt{n: 1, e:0},value);
    } else if action == 'E' {
        waypoint.step(&Pt{n: 0, e:1},value);
    } else if action == 'S' {
        waypoint.step(&Pt{n: -1, e:0},value);
    } else if action == 'W' {
        waypoint.step(&Pt{n: 0, e:-1},value);
    } else {
        ship.step(waypoint,value);
    };
    //println!("{:?}",self);
}


fn part2(filename: &str) -> i32 {
    let ip = parse_input(filename);
    let mut ship = Pt{n: 0, e: 0};
    let mut waypoint = Pt{n: 1, e: 10};

    for instruction in ip {
        update(instruction,& mut waypoint,& mut ship);
    }

    let md = ship.n.abs() + ship.e.abs();
    println!("{:?}",ship);
    println!("aoc 2020 day 12 part 2 for file {filename}, answer = {md}");
    md
}


fn main() {
    part1("test_input");
    part1("input");
    part2("test_input");
    part2("input");
}
