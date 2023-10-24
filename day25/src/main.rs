fn operation(subject_no : u64, mut val : u64) -> u64{
    val *= subject_no;
    val % 20201227
}

fn get_loop_size(subject_no : u64, public_key : u64) -> u64 {
    let mut val = 1;
    let mut loop_size = 0;
    while val != public_key {
        val = operation(subject_no,val);
        loop_size += 1;
    }
    loop_size
}

fn get_key(subject_no : u64, loop_no : u64) -> u64 {
    let mut val = 1;
    for i in 0..loop_no {
        val = operation(subject_no,val);
    }
    val
}

fn day25(door_key : u64, card_key : u64) {
    let door_loop_size = get_loop_size(7,door_key);
    let card_loop_size = get_loop_size(7,card_key);
    let ans1 = get_key(card_key,door_loop_size);
    let ans2 = get_key(door_key,card_loop_size);
    println!("aoc 2020 day 25, {ans1} == {ans2} ?");
}

fn main() {
    let input = [14205034,18047856];
    let test_input = [5764801,17807724];
    day25(test_input[0],test_input[1]);
    day25(input[0],input[1]);
}
