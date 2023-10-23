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

fn get_words(line: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    for word in line.split(" ") {
        let word = word.replace(",", "");
        result.insert(word);
    }
    result
}

fn parse(filename: &str) -> Vec<(BTreeSet<String>, BTreeSet<String>)> {
    let mut result = vec![];
    let lines = get_lines(filename);

    for line in lines {
        let mut line = line.clone();
        line.pop();
        let mut it = line.split(" (contains ");
        let l = it.next().unwrap();
        let r = it.next().unwrap();
        //print!("{l} {r}\n");
        result.push((get_words(l), get_words(r)));
    }
    result
}

fn solve_iteration(
    ip: &Vec<(BTreeSet<String>, BTreeSet<String>)>,
    possible_ingr_by_allergen: &mut BTreeMap<String, BTreeSet<String>>,
    ingr_by_allergen: &mut BTreeMap<String, String>,
    allergen_by_ingr: &mut BTreeMap<String, String>,
) {
    for (iset, aset) in ip {
        for a in aset {
            possible_ingr_by_allergen
                .get_mut(a)
                .unwrap()
                .retain(|x| iset.contains(x));
            let possible = possible_ingr_by_allergen.get(a).unwrap().clone();
            if possible.len() == 1 {
                let i = possible.first().unwrap();
                //println!("{a} {i}");

                ingr_by_allergen.insert(a.clone(), i.clone());
                allergen_by_ingr.insert(i.to_string(), a.clone());
                for ii in possible_ingr_by_allergen.values_mut() {
                    ii.remove(i);
                }
            }
        }
    }
}

fn solve(
    ip: &Vec<(BTreeSet<String>, BTreeSet<String>)>,
) -> (BTreeMap<String, String>, BTreeMap<String, String>) {
    let mut ingr_by_allergen: BTreeMap<String, String> = BTreeMap::new();
    let mut allergen_by_ingr: BTreeMap<String, String> = BTreeMap::new();
    let mut possible_ingr_by_allergen = BTreeMap::new();
    let mut ingrs: BTreeSet<String> = BTreeSet::new();
    let mut allergens: BTreeSet<String> = BTreeSet::new();

    for (iset, aset) in ip {
        for i in iset {
            ingrs.insert(i.clone());
        }
        for a in aset {
            allergens.insert(a.clone());
        }
    }

    let na = allergens.len();

    for a in allergens {
        possible_ingr_by_allergen.insert(a, ingrs.clone());
    }

    while ingr_by_allergen.len() < na {
    //for loop1 in 0..1 {
        //println!("{:?} {:?}", ingr_by_allergen.len(), na);
        solve_iteration(
            ip,
            &mut possible_ingr_by_allergen,
            &mut ingr_by_allergen,
            &mut allergen_by_ingr,
        );
    }
    for i in ingrs {
        if !allergen_by_ingr.contains_key(&i.to_string()) {
            allergen_by_ingr.insert(i.to_string(), "None".to_string());
        }
    }
    (allergen_by_ingr, ingr_by_allergen)
}

fn day21(filename: &str) {
    let ip = parse(filename);
    let (a_by_i, i_by_a) = solve(&ip);
    let mut ans1 = 0;
    for (i, a) in a_by_i {
        if a == "None" {
            for (iset,_aset) in ip.iter() {
                if iset.contains(&i) {
                    ans1 += 1;
                }
            }
        }
    }

    println!("\naoc 2020 day 21 part 1 file {filename} ans = {ans1}");

    for (_a, i) in i_by_a {
        print!("{i},");
    }
    println!("");
}

fn main() {
    day21("test_input");
    day21("input");
}
