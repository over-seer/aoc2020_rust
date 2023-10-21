use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;
use std::cmp::min;
use std::cmp::max;

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

fn parse_input(filename: &str) -> BTreeMap<usize,[[bool;10];10]> {
    let mut result = BTreeMap::new();
    let lines = get_lines(filename);
    let mut itile = 0;
    let mut Border = [[false;10];10];
    let mut irow = 0 as usize;
    for line in lines {
        if line.contains("Tile") {
            let line = &line[5..9];
            //println!("{line}");
            itile = line.parse::<usize>().unwrap();
            Border = [[false;10];10];
            irow = 0;
        } else if line != "" {
            for i in 0..10 as usize {
                if let Some(c) = line.chars().nth(i) {
                    if c == '#' {
                        Border[irow][i] = true;
                    }
                } else {
                    println!("error {line}");
                }
                
            }
            if irow == 9 {
                result.insert(itile,Border.clone());
            }
            irow += 1;
        }
    }
    result
}



struct Border {
    u: String,
    d: String,
    l: String,
    r: String,
}

fn to_bin (b: bool) -> char {
    if b {
        '1'
    } else {
        '0'
    }
}

fn get_borders(pic: &[[bool;10];10]) -> [String;4] {
    let mut u = "".to_string();
    let mut d = "".to_string();
    let mut l = "".to_string();
    let mut r = "".to_string();

    for i in 0..10 {
        u.push(to_bin(pic[0][i]));
        d.push(to_bin(pic[9][i]));
        l.push(to_bin(pic[i][0]));
        r.push(to_bin(pic[i][9]));
    }

    [u,d,l,r].clone()
}

fn apply(bigpic: &mut Vec<Vec<bool>>, smallpic: &[[bool;10];10], ii: usize,jj: usize) {
    for i in 0..8 {
        for j in 0..8 {
            let i1 = i + 1;
            let j1 = j + 1;
            let i2 = ii * 8 + i;
            let j2 = jj * 8 + j;
            *bigpic.get_mut(i2).unwrap().get_mut(j2).unwrap() = smallpic[i1][j1];
        }
    }
}

fn print_index_map(map: &BTreeMap<(usize,usize),usize>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            if let Some(index) = map.get(&(i,j)) {
                print!("{:}   ",index);
            } else {
                print!("----   ");
            }
        }
        println!("");
    }    
}

fn rev(s: &String) -> String {
    s.clone().chars().rev().collect::<String>()
}

fn is_neighbour(b1: &[String; 4], b2: &[String; 4]) -> bool {
    for i in b1 {
        for j in b2 {
            if *i == *j || *i == rev(j) {
                return true;
            }
        }
    }
    false
}

fn is_on_edge(i: usize, n: usize) -> bool {
    i == 0 || i == n-1
}

fn strip_borders(ip: &[[bool;10];10]) -> [[bool;8];8] {
    let mut op = [[false;8];8];
    for i in 0..8 {
        for j in 0..8 {
            op[i][j] = ip[i+1][j+1];
        }
    }
    op
}

//fn flipy(ip: &[[bool])) 

fn orient_top_left(ip: &[[bool;10];10], map: &BTreeMap<String,usize>) -> [[bool;10];10] {
    let op = ip.clone();
    let [u,d,l,r] = get_borders(&ip);
    if map[d] == 1 {

    }
}

fn orient(pic1: &[[bool;10];10], pic2: &[[bool;10];10]) -> [[bool;8];8] {
    strip_borders(pic1)
}

fn part12(filename: &str) {
    let ip = parse_input(filename);
    //println!("{:?}",ip);
    println!("{:}",ip.len());
    let n = (ip.len() as f64).sqrt() as usize;
    println!("n = {n}");
    //let borders = vec![];
    let mut map = BTreeMap::new();
    for (i,pic) in &ip {
        let borders = get_borders(&pic);
        for b in &borders {
            let b = b.clone();
            let brev = rev(&b);
            if b != brev {
                map.entry(brev.clone()).and_modify(|v| *v += 1).or_insert(1);
            }
            map.entry(b.clone()).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    
    //for (s,i) in &map { println!("{s} {i}"); }

    let mut prod = 1 as u64;
    let mut icorner = 0 as usize;
    let mut borders_map = BTreeMap::new();
    let iset = ip.keys().copied().collect::<BTreeSet<_>>();
    let mut cornerset = BTreeSet::new();
    let mut edgeset = BTreeSet::new();
    let mut bodyset = BTreeSet::new();
    for (i,pic) in &ip {
        let borders = get_borders(&pic);
        borders_map.insert(*i,borders.clone());
        let mut count = 0;
        for b in &borders {
            if *map.get(b).unwrap() == 1 {
                count += 1;
            }
        }
        if count == 2 {
            //could be a corner???
            println!("{:} {:?}",i,borders);
            prod *= *i as u64;
            icorner = *i;
            cornerset.insert(*i);
        } else if count == 1 {
            edgeset.insert(*i);
        } else {
            bodyset.insert(*i);
        }
    }
    println!("aoc 2020 day 20 part 1 file {filename} ansswer = {prod}");

    let mut bigpic = vec![vec![false;n*8];n*8];
    let mut index_map = BTreeMap::new();
    let mut checklist = iset.clone();
    for i in 0..n {
        for j in 0..n {
            if i == 0 && j == 0 {
                index_map.insert((i,j),icorner);
                apply(&mut bigpic, &ip[&icorner], i, j);
                checklist.remove(&icorner);
                cornerset.remove(&icorner);
                //borders_map.remove(&icorner);
            } else {
                let mut neighbours: Vec<usize> = Vec::new();
                if i != 0 {
                    neighbours.push(*index_map.get(&(i-1,j)).unwrap());
                }
                if j != 0 {
                    neighbours.push(*index_map.get(&(i,j-1)).unwrap());
                }
                //let checklist = if i == 0 || i == n-1
                let checklist = if is_on_edge(i, n) && is_on_edge(j, n) {
                    &mut cornerset
                } else if is_on_edge(i, n) != is_on_edge(j, n) {
                    &mut edgeset
                } else {
                    &mut bodyset
                };

                let mut found = false;
                for (index,borders) in &borders_map {
                    if checklist.contains(&index) {
                        let mut found_neighbours = true;
                        for ineighbour in &neighbours {
                            let bother = borders_map.get(&ineighbour).unwrap();
                            if !is_neighbour(bother, borders) {
                                found_neighbours = false;
                                break;
                            }
                        }
                        if found_neighbours  {
                            found = true;
                            index_map.insert((i,j),*index);
                            checklist.remove(&index);
                            break;
                        }

                    }
                }
                if !found {
                    print_index_map(&index_map, n);
                    panic!("ERROR - no match {i} {j}");
                } 
            }
        }
    }

    print_index_map(&index_map, n);

    

}

fn part2(filename: &str) {
    let ip = parse_input(filename);
    let mut count = 0;
    for (_i,pic) in ip {
        for i in 1..9 {
            for j in 1..9 {
                if pic[i][j] == true {
                    count += 1;
                }
            }
        }
    }
    count -= 30;
    println!("aoc 2020 part 2 file {filename} answer = {count}");
}

fn main() {
    part12("test_input");
    part12("input");
    part2("test_input");
    part2("input");
}
