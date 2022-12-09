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
    let mut product = 1;
    for tile in tiles.iter() {
        find_neighbors(tile, &sides, &mut neighbors);
        if neighbors.get(&tile.n).unwrap().len() == 2 {
            product *= tile.n;
        }
    }
    println!("Part 1: {}", product);

    //part 2
    let mut big_picture: [[char; 120]; 120] = [['.'; 120]; 120];
    let mut big_i: usize = 0;
    let mut big_j: usize = 0;

    let mut current_tile_id: u64 = 3467; //This was one of the corners
    let mut current_tile = *tiles_map.get(&current_tile_id).unwrap();
    current_tile.rotate_right();

    apply_next_tile(&mut big_i, &mut big_j, &mut big_picture, &current_tile);

    let mut first_tile_in_row = current_tile.clone();
    let mut first_tileid_in_row = current_tile_id.clone();
    for row in 0..=11 {
        for col in 0..=11 {
            if col == 0 {
                if row == 0 {
                    continue;
                } else {
                    find_match_bottom(
                        &mut current_tile,
                        &mut current_tile_id,
                        &neighbors,
                        &mut big_picture,
                        &mut big_i,
                        &mut big_j,
                        &tiles_map,
                    );

                    first_tile_in_row = current_tile.clone();
                    first_tileid_in_row = current_tile_id.clone();
                }
            } else {
                find_match_right(
                    &mut current_tile,
                    &mut current_tile_id,
                    &neighbors,
                    &mut big_picture,
                    &mut big_i,
                    &mut big_j,
                    &tiles_map,
                );
            }
        }
        current_tile = first_tile_in_row;
        current_tile_id = first_tileid_in_row;
    }

    let mut borderless = remove_borders(big_picture);
    let mut monster_locations = borderless.clone();

    count_sea_monsters(borderless, &mut monster_locations);
    flip_horizontal(&mut borderless);
    flip_horizontal(&mut monster_locations);
    count_sea_monsters(borderless, &mut monster_locations);
    flip_vertical(&mut borderless);
    flip_vertical(&mut monster_locations);
    count_sea_monsters(borderless, &mut monster_locations);
    flip_horizontal(&mut borderless);
    flip_horizontal(&mut monster_locations);
    count_sea_monsters(borderless, &mut monster_locations);
    flip_vertical(&mut borderless);
    flip_vertical(&mut monster_locations);

    rotate_right(&mut borderless);
    rotate_right(&mut monster_locations);

    count_sea_monsters(borderless, &mut monster_locations);
    flip_horizontal(&mut borderless);
    flip_horizontal(&mut monster_locations);
    count_sea_monsters(borderless, &mut monster_locations);
    flip_vertical(&mut borderless);
    flip_vertical(&mut monster_locations);
    count_sea_monsters(borderless, &mut monster_locations);
    flip_horizontal(&mut borderless);
    flip_horizontal(&mut monster_locations);
    count_sea_monsters(borderless, &mut monster_locations);

    let mut non_monster_count = 0;
    for i in 0..96{
        for j in 0..96{
            if monster_locations[j][i] == '#' {
                non_monster_count +=1;
            }
        }
    }

    println!("Part 2: {}", non_monster_count);

}

fn find_match_right(
    current_tile: &mut Tile,
    current_tile_id: &mut u64,
    neighbors: &HashMap<u64, HashSet<u64>>,
    big_picture: &mut [[char; 120]; 120],
    big_i: &mut usize,
    big_j: &mut usize,
    tiles_map: &HashMap<u64, Tile>,
) {
    let r = current_tile.right();

    for n in neighbors.get(&current_tile_id).unwrap().iter() {
        let neighbor = tiles_map.get(n).unwrap();

        if r == neighbor.left() {
            *current_tile = neighbor.clone();
            println!("0 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.left_reversed() {
            *current_tile = neighbor.clone();
            current_tile.flip_vertical();
            println!("1 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.bottom() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            println!("2 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.bottom_reversed() {
            *current_tile = neighbor.clone();
            current_tile.flip_horizontal();
            current_tile.rotate_right();
            println!("3 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.right() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.flip_vertical();
            println!("4 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.right_reversed() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            println!("5 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.top() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.flip_vertical();
            println!("6 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if r == neighbor.top_reversed() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.rotate_right();
            println!("7 {n} {:?}", neighbor.left());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        }
    }
}

fn find_match_bottom(
    current_tile: &mut Tile,
    current_tile_id: &mut u64,
    neighbors: &HashMap<u64, HashSet<u64>>,
    big_picture: &mut [[char; 120]; 120],
    big_i: &mut usize,
    big_j: &mut usize,
    tiles_map: &HashMap<u64, Tile>,
) {
    let b = current_tile.bottom();

    for n in neighbors.get(&current_tile_id).unwrap().iter() {
        let neighbor = tiles_map.get(n).unwrap();

        if b == neighbor.left() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.flip_horizontal();
            println!("0 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.left_reversed() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            println!("1 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.bottom() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.flip_horizontal();
            println!("2 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.bottom_reversed() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            println!("3 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.right() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.rotate_right();
            println!("4 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.right_reversed() {
            *current_tile = neighbor.clone();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.rotate_right();
            current_tile.flip_horizontal();
            println!("5 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.top() {
            *current_tile = neighbor.clone();
            println!("6 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        } else if b == neighbor.top_reversed() {
            *current_tile = neighbor.clone();
            current_tile.flip_horizontal();
            println!("7 {n} {:?}", neighbor.top());
            apply_next_tile(big_i, big_j, big_picture, &current_tile);
            *current_tile_id = *n;
            break;
        }
    }
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

fn remove_borders(bp: [[char; 120]; 120]) -> [[char; 96]; 96] {
    let mut borderless = [['.'; 96]; 96];
    let mut b_row = 0;
    let mut b_col = 0;

    for row in 0..120 {
        if row % 10 == 0 || row % 10 == 9 {
            continue;
        }
        for col in 0..120 {
            if col % 10 == 0 || col % 10 == 9 {
                continue;
            }
            borderless[b_row][b_col] = bp[row][col];
            // println!("{} {}", b_row, b_col);
            b_col += 1;
        }
        b_row += 1;
        b_col = 0;
    }

    borderless
}

fn count_sea_monsters(image: [[char; 96]; 96], monster_locations: &mut [[char; 96]; 96]) {
    for col_start in 0..=76 {
        for row_start in 0..=93 {
            let mut tmp: [[char; 20]; 3] = [['.'; 20]; 3];
            for y in 0..20 {
                for x in 0..3 {
                    tmp[x][y] = image[row_start + x][col_start + y];
                }
            }

            if is_sea_monster(tmp) {
                let coords = [
                    (0, 18),
                    (1, 0),
                    (1, 5),
                    (1, 6),
                    (1, 11),
                    (1, 12),
                    (1, 17),
                    (1, 18),
                    (1, 19),
                    (2, 1),
                    (2, 4),
                    (2, 7),
                    (2, 10),
                    (2, 13),
                    (2, 16),
                ];

                for (row, col) in coords {
                    monster_locations[row_start + row][col_start + col] = '0';
                }
            }
        }
    }
}

fn rotate_right(data: &mut [[char; 96]; 96]) {
    let n = 96;

    for i in 0..n {
        for j in 0..i {
            let tmp = data[i][j];
            data[i][j] = data[j][i];
            data[j][i] = tmp;
        }
    }

    // swap columns
    for i in 0..n {
        for j in 0..n / 2 {
            let tmp = data[i][j];
            data[i][j] = data[i][n - j - 1];
            data[i][n - j - 1] = tmp;
        }
    }
}

fn flip_horizontal(data: &mut [[char; 96]; 96]) {
    let n = 96;
    for i in 0..n {
        for j in 0..n / 2 {
            let tmp = data[i][j];
            data[i][j] = data[i][n - j - 1];
            data[i][n - j - 1] = tmp;
        }
    }
}

fn flip_vertical(data: &mut [[char; 96]; 96]) {
    for j in 0..96 / 2 {
        data.swap(j, 96 - j - 1);
    }
}

fn is_sea_monster(tmp: [[char; 20]; 3]) -> bool {
    let coords = [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];

    for (row, col) in coords {
        if tmp[row][col] != '#' {
            return false;
        }
    }
    true
}
