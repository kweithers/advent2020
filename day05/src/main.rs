use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}
fn main() {
    let input = include_str!("day05.txt");
    let mut char_map = HashMap::new();
    char_map.insert('B', '1');
    char_map.insert('F', '0');
    char_map.insert('R', '1');
    char_map.insert('L', '0');

    println!(
        "Part 1: {}",
        input
            .lines()
            .map(|t| convert_ticket_to_seat(t,&char_map))
            .map(|s| s.seat_id())
            .max()
            .unwrap()
    );
    println!("Part 2: {}", find_missing_seat_id(input.lines(), &char_map));
}

fn convert_ticket_to_seat(line: &str, char_map: &HashMap<char,char>) -> Seat {
    let mut bits = String::new();
    for c in line.chars() {
        bits.push(*char_map.get(&c).unwrap())
    }

    let row = usize::from_str_radix(&bits[..7], 2).unwrap();
    let col = usize::from_str_radix(&bits[7..], 2).unwrap();
    Seat { row, col }
}

fn find_missing_seat_id(lines: core::str::Lines, char_map: &HashMap<char,char>) -> usize {
    let mut seat_set = HashSet::new();
    for line in lines {
        seat_set.insert(convert_ticket_to_seat(line, char_map).seat_id());
    }

    for seat in seat_set.iter() {
        if !seat_set.contains(&(*seat + 1)) && seat_set.contains(&(*seat + 2)) {
            return *seat + 1;
        }
    }
    unreachable!()
}
