use std::collections::BTreeMap;
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

fn parse_input(filename: &str) -> BTreeMap<usize, [[bool; 10]; 10]> {
    let mut result = BTreeMap::new();
    let lines = get_lines(filename);
    let mut itile = 0;
    let mut irow = 0 as usize;
    let mut pic = [[false; 10]; 10];
    for line in lines {
        if line.contains("Tile") {
            let line = &line[5..9];
            //println!("{line}");
            itile = line.parse::<usize>().unwrap();
            irow = 0;
            pic = [[false; 10]; 10];
        } else if line != "" {
            for i in 0..10 as usize {
                if let Some(c) = line.chars().nth(i) {
                    if c == '#' {
                        pic[irow][i] = true;
                    }
                } else {
                    println!("error {line}");
                }
            }
            if irow == 9 {
                result.insert(itile, pic.clone());
            }
            irow += 1;
        }
    }
    result
}

fn pic_string<const N: usize, const M: usize>(pic: &[[bool; M]; N]) -> String {
    let mut s = String::new();
    for i in 0..N {
        for j in 0..M {
            s.push(if pic[i][j] { '#' } else { '.' })
        }
        s.push('\n');
    }
    s
}

const TRANS: [(bool, u8); 8] = [
    (false, 0),
    (false, 1),
    (false, 2),
    (false, 3),
    (true, 0),
    (true, 1),
    (true, 2),
    (true, 3),
];

fn get_monster() -> Vec<(usize,usize)> {
    let mut op = vec![];
    let sdragon: [&str; 3] = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    for i in 0..3 {
        for j in 0..20 {
            if sdragon[i].chars().nth(j).unwrap() == '#' {
                op.push((i,j));
            }
        }
    }
    op
}

fn top(pic: &[[bool; 10]; 10]) -> [bool; 10] {
    pic[0].clone()
}

fn bttm(pic: &[[bool; 10]; 10]) -> [bool; 10] {
    pic[9].clone()
}

fn left(pic: &[[bool; 10]; 10]) -> [bool; 10] {
    let mut result = [false; 10];
    for i in 0..10 {
        result[i] = pic[i][0];
    }
    result
}

fn right(pic: &[[bool; 10]; 10]) -> [bool; 10] {
    let mut result = [false; 10];
    for i in 0..10 {
        result[i] = pic[i][9];
    }
    result
}

fn rot90<const N: usize>(ip: &[[bool; N]; N]) -> [[bool; N]; N] {
    let mut op = [[false; N]; N];
    for i in 0..N {
        for j in 0..N {
            op[i][j] = ip[N - 1 - j][i];
        }
    }
    op
}

fn flip<const N: usize>(ip: &[[bool; N]; N]) -> [[bool; N]; N] {
    let mut op = [[false; N]; N];
    for i in 0..N {
        for j in 0..N {
            op[i][N - 1 - j] = ip[i][j];
        }
    }
    op
}

fn orient<const N: usize>(ip: &[[bool; N]; N], is_flip: bool, nrot: u8) -> [[bool; N]; N] {
    let mut op = if is_flip { flip(ip) } else { ip.clone() };
    for _i in 0..nrot {
        op = rot90(&op);
    }
    op
}

fn all_edges(pic: &[[bool; 10]; 10]) -> [[bool; 10]; 4] {
    [top(pic), bttm(pic), left(pic), right(pic)]
}

fn add_to_edge_counter(counter: &mut BTreeMap<[bool; 10], u32>, pic: &[[bool; 10]; 10]) {
    for e1 in all_edges(pic) {
        let mut e2 = e1.clone();
        e2.reverse();
        for e in [&e1, &e2] {
            counter.entry(*e).and_modify(|v| *v += 1).or_insert(1);
        }
    }
}

fn get_edge_count(pics: &BTreeMap<usize, [[bool; 10]; 10]>) -> BTreeMap<[bool; 10], u32> {
    let mut counter = BTreeMap::new();
    for pic in pics.values() {
        add_to_edge_counter(&mut counter, pic);
    }
    counter
}

fn is_corner(counter: &BTreeMap<[bool; 10], u32>, pic: &[[bool; 10]; 10]) -> bool {
    let mut n_outer = 0;
    for e in all_edges(pic) {
        //println!("edge {:?}?",e);
        if *counter.get(&e).unwrap() == 1 {
            n_outer += 1;
        }
    }
    n_outer == 2
}

fn get_corner_ids(pics: &BTreeMap<usize, [[bool; 10]; 10]>) -> Vec<usize> {
    let mut ids = vec![];
    let edge_counter = get_edge_count(pics);
    for (id, pic) in pics {
        if is_corner(&edge_counter, pic) {
            ids.push(*id);
        }
    }
    ids
}

fn usqrt(isq: usize) -> usize {
    let mut i = 1;
    while i * i != isq {
        i += 1;
    }
    i
}

fn add_piece(
    layout: &mut BTreeMap<(usize, usize), [[bool; 10]; 10]>,
    pics: &mut BTreeMap<usize, [[bool; 10]; 10]>,
    i: usize,
    j: usize,
) {
    let mut idij = 0;
    let mut picij = [[false; 10]; 10];
    let mut found = false;
    if (i, j) == (0, 0) {
        let edge_counter = get_edge_count(pics);
        let mut pic_0 = [[false; 10]; 10];
        for (id, pic) in pics.iter() {
            if is_corner(&edge_counter, pic) {
                idij = *id;
                pic_0 = pic.clone();
                break;
            }
        }
        for (f, r) in TRANS {
            let pic = orient(&pic_0, f, r);
            if *edge_counter.get(&top(&pic)).unwrap() == 1
                && *edge_counter.get(&left(&pic)).unwrap() == 1
            {
                picij = pic.clone();
                break;
            }
        }
    } else if i == 0 {
        let nbr = right(&layout.get(&(i, j - 1)).unwrap());
        for (id, pic) in pics.iter() {
            for (f, r) in TRANS {
                let pic = orient(pic, f, r);
                if left(&pic) == nbr {
                    idij = *id;
                    picij = pic.clone();
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    } else {
        let nbr = bttm(&layout.get(&(i - 1, j)).unwrap());
        for (id, pic) in pics.iter() {
            for (f, r) in TRANS {
                let pic = orient(pic, f, r);
                if top(&pic) == nbr {
                    idij = *id;
                    picij = pic.clone();
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    layout.insert((i, j), picij);
    pics.remove(&idij);
}

fn add_subsection<const N: usize>(
    big_pic: &mut [[bool; N]; N],
    pic: &[[bool; 10]; 10],
    ij: (usize, usize),
) {
    let (i, j) = ij;
    for ii in 0..8 {
        for jj in 0..8 {
            big_pic[i * 8 + ii][j * 8 + jj] = pic[ii + 1][jj + 1];
        }
    }
}

fn look_for_monsters<const N: usize>(pic : &[[bool;N];N]) -> Option<usize> {
    let monster = get_monster();
    let mut coords = BTreeSet::new();
    let mut hic_sont_dracones = false;
    for (f,r) in TRANS {
        let pic = orient(pic,f,r);
        coords.clear();
        for i in 0..N-3 {
            for j in 0..N-20 {
                let mut found = true;
                for (di,dj) in &monster {
                    let ii = i + di;
                    let jj = j + dj;
                    if !pic[ii][jj] {
                        found = false;
                        break;
                    }
                }
                if found {
                    for (di,dj) in &monster {
                        let ii = i + di;
                        let jj = j + dj;
                        coords.insert((ii,jj));
                    }
                    hic_sont_dracones = true;
                }
            }
        }
    }
    if hic_sont_dracones {
        let mut n = 0;
        for i in 0..N {
            for j in 0..N {
                if pic[i][j] {
                    n += 1;
                }
            }
        }
        Some(n - coords.len())
    } else {
        None
    }
}

fn day20<const N: usize>(filename: &str) {
    let pics = parse_input(filename);
    for pic in pics.values() {
        let pstr = pic_string(pic);
        print!("{pstr}\n");
    }
    let corner_ids = get_corner_ids(&pics);
    let mut ans = 1;
    for id in corner_ids {
        print!("{id}, ");
        ans *= id;
    }
    println!("aoc 2020 day 20 file {filename} ans = {ans}");
    let nside = usqrt(pics.len());
    println!("{nside}x{nside}");

    let mut layout = BTreeMap::new();
    let mut big_pic = [[false; N]; N];
    let mut remaining = pics.clone();

    for i in 0..N / 8 {
        for j in 0..N / 8 {
            add_piece(&mut layout, &mut remaining, i, j);
        }
    }

    for (ij, pic) in &layout {
        add_subsection(&mut big_pic, pic, *ij);
    }

    let pstr = pic_string(&big_pic);
    print!("{pstr}\n");

    if let Some(ans) = look_for_monsters(&big_pic) {
        println!("aoc 2020 day 20 part 2 file {filename} ans = {ans}");
    }

}

fn main() {
    day20::<24>("test_input"); //24 = 3*8
    day20::<96>("input"); //96 = 12*8
}
