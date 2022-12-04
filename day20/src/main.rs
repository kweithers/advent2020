use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy)]
struct Tile {
    n: u64,
    data: [char; 100], // First ten are row 1, last ten are row 10
}

impl Tile {
    fn top(&self) -> [char; 10] {
        let mut side = ['.'; 10];
        for i in 0..10 {
            side[i] = self.data[i];
        }
        side
    }
    fn top_reversed(&self) -> [char; 10] {
        let mut side = self.top();
        side.reverse();
        side
    }

    fn bottom(&self) -> [char; 10] {
        let mut side = ['.'; 10];
        for i in 0..10 {
            side[i] = self.data[90 + i];
        }
        side
    }
    fn bottom_reversed(&self) -> [char; 10] {
        let mut side = self.bottom();
        side.reverse();
        side
    }

    fn left(&self) -> [char; 10] {
        let mut side = ['.'; 10];
        for i in 0..10 {
            side[i] = self.data[10 * i];
        }
        side
    }
    fn left_reversed(&self) -> [char; 10] {
        let mut side = self.left();
        side.reverse();
        side
    }

    fn right(&self) -> [char; 10] {
        let mut side = ['.'; 10];
        for i in 0..10 {
            side[i] = self.data[9 + 10 * i];
        }
        side
    }
    fn right_reversed(&self) -> [char; 10] {
        let mut side = self.right();
        side.reverse();
        side
    }

    fn rotate_right(&mut self) {
        let mut new_data = ['.'; 100];

        for i in 0..10 {
            for j in 0..10 {
                new_data[i * 10 + j] = self.data[(10 - j - 1) * 10 + i];
            }
        }
        self.data = new_data;
    }

    fn flip_horizontal(&mut self) {
        let mut new_data = ['.'; 100];

        for i in 0..10 {
            for j in 0..10 {
                new_data[i * 10 + j] = self.data[i * 10 + 9 - j]
            }
        }
        self.data = new_data;
    }

    fn flip_vertical(&mut self) {
        let mut new_data = ['.'; 100];

        for i in 0..10 {
            for j in 0..10 {
                new_data[i * 10 + j] = self.data[(9 - i) * 10 + j]
            }
        }
        self.data = new_data;
    }
}

fn main() {
    let input = include_str!("input.txt");
    let string_tiles = input.split("\n\n");
    let mut tiles_map: HashMap<u64, Tile> = HashMap::new();
    let tiles: Vec<Tile> = string_tiles
        .map(|s| parse_tile(s, &mut tiles_map))
        .collect();

    let mut sides: HashMap<u64, Vec<[char; 10]>> = HashMap::new();
    for tile in tiles.iter() {
        let mut v = Vec::new();
        v.push(tile.top());
        v.push(tile.top_reversed());
        v.push(tile.bottom());
        v.push(tile.bottom_reversed());
        v.push(tile.right());
        v.push(tile.right_reversed());
        v.push(tile.left());
        v.push(tile.left_reversed());
        sides.insert(tile.n, v);
    }

    let mut neighbors: HashMap<u64, HashSet<u64>> = HashMap::new();
    for tile in tiles.iter() {
        find_neighbors(tile, &sides, &mut neighbors);
        // if neighbors.get(&tile.n).unwrap().len() == 2 {
        //     // println!("{}", tile.n);
        // }
    }

    //part 2
    let mut big_picture: [[char; 120]; 120] = [['.'; 120]; 120];
    let mut big_i: usize = 0;
    let mut big_j: usize = 0;
    
    let mut current_tile_id: u64 = 1249; //This was one of the corners
    let mut current_tile = *tiles_map.get(&current_tile_id).unwrap();
    current_tile.rotate_right();
    current_tile.rotate_right();
    apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
    
    for i in 0..12 {
        let r = current_tile.right();

        for n in neighbors.get(&current_tile_id).unwrap().iter() {
            let neighbor = tiles_map.get(n).unwrap();

            if r == neighbor.left() {
                current_tile = neighbor.clone();
                println!("0 {n} {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.left_reversed() {
                current_tile = neighbor.clone();
                current_tile.flip_vertical();
                println!("1 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture,&current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.bottom() {
                current_tile = neighbor.clone();
                current_tile.rotate_right();
                println!("2 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.bottom_reversed() {
                current_tile = neighbor.clone();
                current_tile.flip_horizontal();
                current_tile.rotate_right();
                println!("3 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.right() {
                current_tile = neighbor.clone();
                current_tile.rotate_right();
                current_tile.rotate_right();
                current_tile.flip_vertical();
                println!("4 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.right_reversed() {
                current_tile = neighbor.clone();
                current_tile.rotate_right();
                current_tile.rotate_right();
                println!("5 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.top() {
                current_tile = neighbor.clone();
                current_tile.rotate_right();
                current_tile.rotate_right();
                current_tile.rotate_right();
                current_tile.flip_vertical();
                println!("6 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            } else if r == neighbor.top_reversed() {
                current_tile = neighbor.clone();
                current_tile.rotate_right();
                current_tile.rotate_right();
                current_tile.rotate_right();
                println!("7 {:?}", neighbor.left());
                apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);
                current_tile_id = *n;
                break;
            }
        }
    }
    println!("{:?}", big_picture);
}

fn parse_tile(s: &str, map: &mut HashMap<u64, Tile>) -> Tile {
    let mut lines = s.lines();
    let mut index = lines.next().unwrap().strip_suffix(":").unwrap().split(" ");
    let _ = index.next();
    let n = index.next().unwrap().parse::<u64>().unwrap();
    let mut data = ['.'; 100];
    for (j, line) in lines.enumerate() {
        for (i, ch) in line.chars().enumerate() {
            data[10 * j + i] = ch;
        }
    }
    map.insert(n, Tile { n, data });
    Tile { n, data }
}

fn find_neighbors(
    this_tile: &Tile,
    tiles: &HashMap<u64, Vec<[char; 10]>>,
    neighbors: &mut HashMap<u64, HashSet<u64>>,
) {
    let these_sides = tiles.get(&this_tile.n).unwrap();
    let mut v = HashSet::new();

    for (i, sides) in tiles {
        if *i == this_tile.n {
            continue;
        }
        for side in sides {
            if these_sides.contains(side) {
                v.insert(*i);
            }
        }
    }
    neighbors.insert(this_tile.n, v);
}

fn apply_next_tile(i: &mut usize, j: &mut usize, bp: &mut [[char; 120]; 120], tile: &Tile) {
    for y in 0..10 {
        for x in 0..10 {
            bp[*j + y][*i + x] = tile.data[y * 10 + x];
        }
    }
    *i += 10;
    if *i == 120 {
        *j += 10;
        *i = 0;
    }
}
