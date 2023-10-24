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

fn parse_input(filename: &str) -> BTreeSet<(i64, i64, i64)> {
    let mut result = BTreeSet::new();
    let lines = get_lines(filename);
    let z = 0 as i64;
    let mut y = 0 as i64;
    for line in lines {
        let mut x = 0 as i64;
        for c in line.chars() {
            if c == '#' {
                result.insert((x, y, z));
            }
            x += 1;
        }
        y += 1;
    }
    result
}

fn neighbours(pt: &(i64, i64, i64)) -> BTreeSet<(i64, i64, i64)> {
    let mut s = BTreeSet::new();
    let (x, y, z) = pt;
    for i in x - 1..x + 2 {
        for j in y - 1..y + 2 {
            for k in z - 1..z + 2 {
                s.insert((i, j, k));
            }
        }
    }
    s.remove(pt);
    s
}

fn update(grid: &BTreeSet<(i64, i64, i64)>) -> BTreeSet<(i64, i64, i64)> {
    let mut new_grid: BTreeSet<(i64, i64, i64)> = BTreeSet::new();

    for pt1 in grid {
        let mut sum = 0;
        for pt2 in &neighbours(&pt1) {
            if grid.contains(pt2) {
                sum += 1;
            } else if !new_grid.contains(pt2) {
                let mut sumi = 0;
                for pt3 in &neighbours(pt2) {
                    if grid.contains(pt3) {
                        sumi += 1;
                        if sumi == 4 {
                            break;
                        }
                    }
                }
                if sumi == 3 {
                    new_grid.insert(pt2.clone());
                }
            }
        }
        //println!("pt {:?} has {:} neighbours",pt1,sum);
        if sum == 2 || sum == 3 {
            new_grid.insert(pt1.clone());
        }
    }
    new_grid
}

fn _print_grid(grid: &BTreeSet<(i64, i64, i64)>) {
    let xmin = 0;
    let ymin = 0;
    let zmin = 0;
    let mut xmax = 0;
    let mut ymax = 0;
    let mut zmax = 0;

    for (x, y, z) in grid {
        xmax = max(xmax, *x);
        ymax = max(ymax, *y);
        zmax = max(zmax, *z);
    }

    for z in zmin..zmax + 1 {
        for y in ymin..ymax + 1 {
            for x in xmin..xmax + 1 {
                let c = if grid.contains(&(x, y, z)) { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
        println!();
    }
}

fn part1(filename: &str) {
    let ip = parse_input(filename);
    let mut grid = ip.clone();

    //print_grid(&grid);
    for _i in 0..6 {
        grid = update(&grid);
        //print_grid(&grid);
    }
    let sum = grid.len();
    println!("aoc 2020 day 17 part 1 file {filename}, answer = {sum}")
}

fn parse_input2(filename: &str) -> BTreeSet<(i64, i64, i64, i64)> {
    let mut result = BTreeSet::new();
    let lines = get_lines(filename);
    let z = 0 as i64;
    let mut y = 0 as i64;
    for line in lines {
        let mut x = 0 as i64;
        for c in line.chars() {
            if c == '#' {
                result.insert((x, y, z, z));
            }
            x += 1;
        }
        y += 1;
    }
    result
}

fn neighbours2(pt: &(i64, i64, i64, i64)) -> BTreeSet<(i64, i64, i64, i64)> {
    let mut s = BTreeSet::new();
    let (x, y, z, w) = pt;
    for i in x - 1..x + 2 {
        for j in y - 1..y + 2 {
            for k in z - 1..z + 2 {
                for l in w - 1..w + 2 {
                    s.insert((i, j, k, l));
                }
            }
        }
    }
    s.remove(pt);
    s
}

fn update2(grid: &BTreeSet<(i64, i64, i64, i64)>) -> BTreeSet<(i64, i64, i64, i64)> {
    let mut new_grid: BTreeSet<(i64, i64, i64, i64)> = BTreeSet::new();

    for pt1 in grid {
        let mut sum = 0;
        for pt2 in &neighbours2(&pt1) {
            if grid.contains(pt2) {
                sum += 1;
            } else if !new_grid.contains(pt2) {
                let mut sumi = 0;
                for pt3 in &neighbours2(pt2) {
                    if grid.contains(pt3) {
                        sumi += 1;
                        if sumi == 4 {
                            break;
                        }
                    }
                }
                if sumi == 3 {
                    new_grid.insert(pt2.clone());
                }
            }
        }
        //println!("pt {:?} has {:} neighbours",pt1,sum);
        if sum == 2 || sum == 3 {
            new_grid.insert(pt1.clone());
        }
    }
    new_grid
}

fn part2(filename: &str) {
    let ip = parse_input2(filename);
    let mut grid = ip.clone();

    //print_grid(&grid);
    for _i in 0..6 {
        grid = update2(&grid);
        //print_grid(&grid);
    }
    let sum = grid.len();
    println!("aoc 2020 day 17 part 2 file {filename}, answer = {sum}")
}

fn main() {
    part1("test_input");
    part1("input");
    part2("test_input");
    part2("input");
}
