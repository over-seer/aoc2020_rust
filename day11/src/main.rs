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

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let mut result = vec![];
    let ip = get_lines(filename);
    for line in ip {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }
    result
}

fn _print_map(map: &Vec<Vec<char>>) {
    for v in map {
        for c in v {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn count_neighbours(map: &Vec<Vec<char>>, nx: usize, ny: usize, x0: usize, y0: usize) -> usize {
    let mut n = 0;
    let xmin = if x0 == 0 { x0 } else { x0 - 1 };
    let xmax = if x0 == nx - 1 { x0 } else { x0 + 1 };
    let ymin = if y0 == 0 { y0 } else { y0 - 1 };
    let ymax = if y0 == ny - 1 { y0 } else { y0 + 1 };

    for x in xmin..xmax + 1 {
        for y in ymin..ymax + 1 {
            if map[y][x] == '#' {
                n += 1;
            };
        }
    }

    if map[y0][x0] == '#' {
        n -= 1;
    }
    n
}

fn in_bounds(nx: i32, ny: i32, x: i32, y: i32) -> bool {
    x >= 0 && x < nx && y >= 0 && y < ny
}

fn count_in_sight(map: &Vec<Vec<char>>, nx: usize, ny: usize, x0: usize, y0: usize) -> usize {
    let mut n = 0;
    let mut x = [x0 as i32; 8];
    let mut y = [y0 as i32; 8];
    let mut is_in = [true; 8];

    while is_in.iter().any(|&b| b) {
        x[0] -= 1;
        y[0] -= 1;
        x[1] += 1;
        y[1] -= 1;
        x[2] += 1;
        y[2] += 1;
        x[3] -= 1;
        y[3] += 1;
        x[4] -= 1;
        x[5] += 1;
        y[6] -= 1;
        y[7] += 1;

        for i in 0..8 {
            if is_in[i] && in_bounds(nx as i32, ny as i32, x[i], y[i]) {
                if map[y[i] as usize][x[i] as usize] != '.' {
                    is_in[i] = false;
                    if map[y[i] as usize][x[i] as usize] == '#' {
                        n += 1;
                    }
                }
            } else {
                is_in[i] = false;
            }
        }
    }
    n
}

fn update(old_map: &Vec<Vec<char>>, part: usize) -> Vec<Vec<char>> {
    let mut new_map = old_map.clone();
    let ny = old_map.len();
    let nx = old_map[0].len();
    let tolerance = if part == 1 { 4 } else { 5 };
    for y in 0..ny {
        for x in 0..nx {
            if old_map[y][x] != '.' {
                let nn = if part == 1 {
                    count_neighbours(old_map, nx, ny, x, y)
                } else {
                    count_in_sight(old_map, nx, ny, x, y)
                };
                //if part == 2 { println!("{nn} neighbours"); }

                if nn == 0 {
                    new_map[y][x] = '#';
                } else if nn >= tolerance {
                    new_map[y][x] = 'L';
                }
            }
        }
    }
    new_map
}

fn go(filename: &str, part: usize) -> usize {
    let mut old_map = parse_input(filename);
    let mut converged = false;
    while !converged {
        let new_map = update(&old_map, part);
        converged = new_map == old_map;
        old_map = new_map;
    }

    //print_map(&old_map);

    let mut n = 0;
    for c in old_map.into_iter().flatten() {
        if c == '#' {
            n += 1;
        }
    }

    println!("aoc 2020 day 11 part {part} for file {filename}, answer = {n}");
    n
}

fn main() {
    go("test_input", 1);
    go("input", 1);
    go("test_input", 2);
    go("input", 2);

    //let mut map1 = parse_input("test_input");
    for _i in 0..4 {
        //let map2 = update(&map1,2);
        //print_map(&map2);
        //map1 = map2;
    }
}
