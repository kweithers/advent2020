/*
* First solution runs in 1500ms
* Uses a hashmap to store (num, previous_spoken_turn) pairs.

use std::collections::HashMap;
fn main() {
    let input = vec![8,13,1,0,18,9];
    let mut turn_map = HashMap::new();
    for (turn, n) in input.iter().enumerate() {
        turn_map.insert(*n, turn+1);
    }
    let mut previous_number = *input.last().unwrap();

    for turn in input.len()+1..30000001 {
        // If previously spoken
        if let Some(val) = turn_map.get(&previous_number) {
            let diff = (turn - 1) - val;
            turn_map.insert(previous_number, turn-1);
            //End Turn; Speak diff
            previous_number = diff;
        //If not previously spoken
        } else {
            turn_map.insert(previous_number, turn-1);
            //End Turn; Speak 0
            previous_number = 0;
        }

        if turn == 2020 {
            println!("Part 1: {}", previous_number);
        }
    }
    println!("Part 2: {}", previous_number);
}
*/

// 850ms using a vec instead (to avoid hashing)
fn main() {
    let input = vec![8, 13, 1, 0, 18, 9];
    let mut turn_vec: Vec<isize> = vec![-1; 30000001];
    for (turn, n) in input.iter().enumerate() {
        turn_vec[*n] = turn as isize + 1;
    }
    let mut previous_number = *input.last().unwrap();

    for turn in input.len() + 1..30000001 {
        let spoken = match turn_vec[previous_number] {
            -1 => 0,
            n => (turn as isize - 1) - n,
        };

        turn_vec[previous_number as usize] = turn as isize - 1;
        previous_number = spoken as usize;

        if turn == 2020 {
            println!("Part 1: {}", previous_number);
        }
    }
    println!("Part 2: {}", previous_number);
}
