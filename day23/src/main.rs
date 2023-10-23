use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn parse(ip: &str, ncups: usize) -> (usize, BTreeMap<usize, usize>) {
    let mut cups = BTreeMap::new();
    let mut vip = vec![];
    for c in ip.chars() {
        vip.push(c.to_digit(10).unwrap() as usize);
    }
    let nip = vip.len();
    for i in 0..nip - 1 {
        cups.insert(vip[i], vip[i + 1]);
    }
    if ncups > nip {
        cups.insert(*vip.last().unwrap(), nip + 1);
    } else {
        cups.insert(*vip.last().unwrap(), *vip.first().unwrap());
    }
    for i in nip + 1..ncups + 1 {
        if i == ncups {
            cups.insert(i, *vip.first().unwrap());
        } else {
            cups.insert(i, i + 1);
        }
    }
    (vip[0], cups)
}

fn move_cups(cups: &mut BTreeMap<usize, usize>, start: usize, nmoves: usize) {
    let mut current = start;
    for i in 0..nmoves {
        let r1 = cups.get(&current).unwrap();
        let i1 = *r1;
        let r2 = cups.get(&i1).unwrap();
        let i2 = *r2;
        let r3 = cups.get(&i2).unwrap();
        let i3 = *r3;
        let r4 = cups.get(&i3).unwrap();
        let i4 = *r4;
        let mut dest = if current == 1 {
            cups.len()
        } else {
            current - 1
        };
        let mut is_dest_ok = false;
        while !is_dest_ok {
            if dest == i1 || dest == i2 || dest == i3 {
                dest -= 1;
                if dest == 0 {
                    dest = cups.len();
                }
            } else {
                is_dest_ok = true;
            }
        }
        let rdestr = cups.get(&dest).unwrap();
        let idestr = *rdestr;
        let moves = [(dest,i1),(i3,idestr),(current,i4)];
        for (k,v) in moves {
            *cups.get_mut(&k).unwrap() = v;
        }
        current = i4;
    }
}

fn part1(ip: &str) {
    let (ifirst, mut cups) = parse(ip, 9);
    println!("{ifirst} {:?}", cups);
    move_cups(&mut cups, ifirst, 100);
    println!("{ifirst} {:?}", cups);
    let mut icup = 1;
    for i in 0..8 {
        icup = *cups.get(&icup).unwrap();
        print!("{icup}");
    }
    println!(" <- answer to part1 for {}",ip);
}

fn part2(ip: &str) {
    let (ifirst, mut cups) = parse(ip, 1000000);
    move_cups(&mut cups, ifirst, 10000000);
    let mut icup = 1;
    let mut ans = 1;
    for i in 0..2 {
        icup = *cups.get(&icup).unwrap();
        print!("{icup},");
        ans *= icup;
    }
    println!(" ... answer to part2 for {} is {}",ip,ans);
}

fn main() {
    let test_input = "389125467";
    let input = "871369452";
    //println!("{input}");
    part1(test_input);
    part1(input);
    part2(test_input);
    part2(input);
}
