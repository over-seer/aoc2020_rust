use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
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

fn parse(filename: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let ip = get_lines(filename);
    let mut d1 = VecDeque::new();
    let mut d2 = VecDeque::new();
    let mut d = &mut d1;
    for line in ip {
        if line == "Player 2:" {
            d = &mut d2;
        } else if !line.is_empty() && line != "Player 1:" {
            //d.push_back(line.parse::<usize>().unwrap());
            if let Ok(i) = line.parse::<usize>() {
                d.push_back(i);
            } else {
                println!("{line}");
            }
        }
    }
    (d1, d2)
}

fn round(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) {
    let card1 = deck1.pop_front().unwrap();
    let card2 = deck2.pop_front().unwrap();
    if card1 > card2 {
        deck1.push_back(card1);
        deck1.push_back(card2);
    } else {
        deck2.push_back(card2);
        deck2.push_back(card1);
    }
}

fn part1(filename: &str) {
    let (mut deck1, mut deck2) = parse(filename);
    while !deck1.is_empty() && !deck2.is_empty() {
        round(&mut deck1, &mut deck2);
    }
    let winning_deck = if deck1.is_empty() { deck2 } else { deck1 };
    let ans = score(winning_deck);
    println!("{ans}");
}

fn play2(mut cards: (VecDeque<usize>, VecDeque<usize>)) -> (usize, VecDeque<usize>) {
    let mut record = BTreeSet::new();
    loop {
        if let Some(winner) = round2(&mut cards, &mut record) {
            if winner == 1 {
                return (winner, cards.0);
            } else {
                return (winner, cards.1);
            }
        }
    }
}

fn round2(
    cards: &mut (VecDeque<usize>, VecDeque<usize>),
    record: &mut BTreeSet<(VecDeque<usize>, VecDeque<usize>)>,
) -> Option<usize> {
    //player 1 is winner if these decks already seen in this (sub)game
    if !record.contains(cards) {
        record.insert(cards.clone());
    } else {
        return Some(1);
    }
    let (deck1, deck2) = cards;
    //println!("{:?} {:?}", deck1, deck2);
    let card1 = deck1.pop_front().unwrap();
    let card2 = deck2.pop_front().unwrap();

    let winner = if deck1.len() >= card1 && deck2.len() >= card2 {
        let cards = (
            deck1.range(0..card1).copied().collect::<VecDeque<_>>(),
            deck2.range(0..card2).copied().collect::<VecDeque<_>>(),
        );
        play2(cards).0
    } else if card1 > card2 {
        1
    } else {
        2
    };

    if winner == 1 {
        deck1.push_back(card1);
        deck1.push_back(card2);
    } else {
        deck2.push_back(card2);
        deck2.push_back(card1);
    }

    if deck1.is_empty() {
        return Some(2);
    }

    if deck2.is_empty() {
        return Some(1);
    }

    None
}

fn score(mut winning_deck: VecDeque<usize>) -> usize {
    let mut ans = 0;
    let n = winning_deck.len();
    for i in 0..n {
        ans += (i + 1) * winning_deck.pop_back().unwrap();
    }
    ans
}

fn part2(filename: &str) {
    let cards = parse(filename);
    let (winner, deck) = play2(cards.clone());
    let ans = score(deck.clone());

    println!("{:?} {:?}", winner, deck);
    println!("{ans}");
}

fn main() {
    part1("test_input");
    part2("test_input");
    part1("input");
    part2("input");
}
