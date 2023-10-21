use std::collections::BTreeMap;

fn play(nos: &[u64]) -> u64 {
    let last = nos.last().unwrap();
    for i in 1..nos.len() {
        let j = nos.len() - i - 1;
        if nos[j] == *last {
            return i as u64;
        }
    }
    return 0;
}

fn part1(ip: Vec<u64>, n: u64) {
    let mut nos = ip.clone();
    for _i in 0..n as usize - ip.len() {
        nos.push(play(&nos[..]));
    }
    let ans = nos.last().unwrap();
    //println!("{:?}",nos);
    println!("aoc 2020 day 15 part 1 for ip {:?} ans = {:}",ip,ans);
}

#[derive(Debug)]
struct Mem {
    positions : BTreeMap<u64,u64>,
    last_val : u64,
    last_pos : u64
}

impl Mem {
    pub fn play(&mut self) -> u64 {
        let new_last_pos = self.last_pos + 1;

        let pos = self.positions.get(&self.last_val);
        let new_last_val = if let Some(pos) = pos {
            self.last_pos - *pos
        } else {
            0
        };
        self.positions.insert(self.last_val,self.last_pos);
        self.last_pos = new_last_pos;
        self.last_val = new_last_val;
        self.last_val
    }
}

fn part2(ip: &Vec<u64>, n: u64) {
    let mut map = BTreeMap::new();
    for i in 0..(ip.len() - 1) as u64 {
        map.insert(ip[i as usize],i);
    }

    let mut game = Mem{
        positions: map,
        last_val: *ip.last().unwrap(),
        last_pos: ip.len() as u64 - 1
    };

    let mut ans = 0 as u64;
    for _i in 0..n as usize - ip.len() {
        ans = game.play();
    }
    println!("aoc 2020 day 15 part 2 ip {:?}, ans = {:}",ip,ans);
}


fn main() {
    let test_input = vec![0 as u64,3,6];
    let input = vec![2 as u64,15,0,9,1,20];
    part1(test_input.clone(),2020);
    part1(vec![1,3,2],2020);
    part1(vec![2,1,3],2020);
    part1(vec![1,2,3],2020);
    part1(input.clone(),2020);


    part2(&test_input,2020);
    part2(&vec![1,3,2],2020);
    part2(&vec![2,1,3],2020);
    part2(&vec![1,2,3],2020);
    part2(&input,2020);
    part2(&input,30000000);
}
