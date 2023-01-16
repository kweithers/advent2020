/* Really only three directions..
1. East = -West
2. NorthEast = -SouthWest
3. SouthEast = -NorthWest

Can further reduce to two, since:
East = NorthEast + SouthEast
West = -East = -NorthEast - SouthEast

So, any tile location can be uniquely represented by (+ or -) steps in these two directions :D */

use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Location {
    ne: i32,
    se: i32,
}
impl Add for Location {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ne: self.ne + other.ne,
            se: self.se + other.se,
        }
    }
}
fn main() {
    let input = include_str!("input.txt");
    let locations = input
        .lines()
        .map(|s| parse_path(s))
        .collect::<Vec<Location>>();

    let mut location_counter = HashMap::new();
    for location in locations {
        match location_counter.get(&location) {
            Some(count) => {
                location_counter.insert(location, count + 1);
            }
            None => {
                location_counter.insert(location, 1);
            }
        }
    }

    let n_flipped: usize = location_counter
        .iter()
        .filter(|&(_k, v)| v % 2 == 1)
        .count();
    println!("Part 1: {n_flipped}");

    // Part 2
    let mut flipped_tiles: HashSet<Location> = location_counter
        .iter()
        .filter_map(|(k, v)| match v % 2 {
            1 => Some(k.to_owned()),
            0 => None,
            _ => unreachable!(),
        })
        .collect();

    for _steps in 0..100 {
        flipped_tiles = next_step(flipped_tiles);
    }
    println!("Part 2: {}", flipped_tiles.len());
}

fn parse_path(s: &str) -> Location {
    let tokens = vec![
        "e".to_owned(),
        "ne".to_owned(),
        "se".to_owned(),
        "w".to_owned(),
        "sw".to_owned(),
        "nw".to_owned(),
    ];

    let mut location = Location { ne: 0, se: 0 };
    let mut current_item = "".to_owned();
    for char in s.chars() {
        current_item.push_str(char.to_string().as_str());

        if tokens.contains(&current_item) {
            apply_item(&mut location, current_item);
            current_item = "".to_owned();
        }
    }
    location
}

fn apply_item(l: &mut Location, token: String) {
    match token.as_str() {
        "e" => {
            l.ne += 1;
            l.se += 1
        }
        "ne" => l.ne += 1,
        "se" => l.se += 1,
        "w" => {
            l.ne -= 1;
            l.se -= 1;
        }
        "sw" => l.ne -= 1,
        "nw" => l.se -= 1,
        _ => unreachable!("invalid direction"),
    };
}

// Returns a set of flipped/black tiles
fn next_step(flipped_tiles: HashSet<Location>) -> HashSet<Location> {
    // kv pairs for location, number of flipped neighbors
    let mut neighbor_counter = HashMap::new();
    let directions = vec![
        Location { ne: 1, se: 0 },
        Location { ne: -1, se: 0 },
        Location { ne: 0, se: 1 },
        Location { ne: 0, se: -1 },
        Location { ne: 1, se: 1 },
        Location { ne: -1, se: -1 },
    ];

    for location in flipped_tiles.iter() {
        for direction in directions.iter() {
            let neighbor = *location + *direction;
            match neighbor_counter.get(&neighbor) {
                Some(count) => {
                    neighbor_counter.insert(neighbor, count + 1);
                }
                None => {
                    neighbor_counter.insert(neighbor, 1);
                }
            }
        }
    }

    let mut result = flipped_tiles.clone();

    // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    for tile in flipped_tiles.iter() {
        match neighbor_counter.get(tile) {
            Some(value) => match value {
                1 | 2 => continue,
                _ => {
                    result.remove(tile);
                } //more than 2
            },
            None => {
                result.remove(tile);
            } //zero
        }
    }

    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    for (tile, value) in neighbor_counter.iter() {
        if !flipped_tiles.contains(tile) {
            match value {
                2 => {
                    result.insert(*tile);
                }
                _ => continue,
            }
        }
    }

    result
}
