use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}
fn main() {
    let input = include_str!("day17.txt");
    let mut m: HashMap<Point, bool> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            m.insert(
                Point {
                    x: col as i64,
                    y: row as i64,
                    z: 0,
                },
                val == '#',
            );
        }
    }

    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (-1, 8, -1, 8, -1, 1);
    for _ in 1..=6 {
        let mut new_m: HashMap<Point, bool> = HashMap::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    let this_point = Point { x, y, z };
                    new_m.insert(this_point, determine_new_state_3d(&this_point, &m));
                }
            }
        }
        m.clear();
        m.extend(new_m.into_iter());
        x_min -= 1;
        x_max += 1;
        y_min -= 1;
        y_max += 1;
        z_min -= 1;
        z_max += 1;
    }
    println!("Part 1: {}", m.iter().filter(|(_, &v)| v).count());

    // Part 2 - 4 Dimensions
    let mut m: HashMap<Point4D, bool> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            m.insert(
                Point4D {
                    x: col as i64,
                    y: row as i64,
                    z: 0,
                    w: 0,
                },
                val == '#',
            );
        }
    }

    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max, mut w_min, mut w_max) =
        (-1, 8, -1, 8, -1, 1, -1, 1);
    for _ in 1..=6 {
        let mut new_m: HashMap<Point4D, bool> = HashMap::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    for w in w_min..=w_max {
                        let this_point = Point4D { x, y, z, w };
                        new_m.insert(this_point, determine_new_state_4d(&this_point, &m));
                    }
                }
            }
        }
        m.clear();
        m.extend(new_m.into_iter());
        x_min -= 1;
        x_max += 1;
        y_min -= 1;
        y_max += 1;
        z_min -= 1;
        z_max += 1;
        w_min -= 1;
        w_max += 1;
    }
    println!("Part 2: {}", m.iter().filter(|(_, &v)| v).count());
}

fn count_active_neighbors_3d(point: &Point, map: &HashMap<Point, bool>) -> i64 {
    let mut active_neighbors: i64 = 0;
    for x in [-1, 0, 1 as i64] {
        for y in [-1, 0, 1 as i64] {
            for z in [-1, 0, 1 as i64] {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let this_neighbor = Point {
                    x: point.x + x,
                    y: point.y + y,
                    z: point.z + z,
                };
                if let Some(&v) = map.get(&this_neighbor) {
                    active_neighbors += v as i64;
                };
            }
        }
    }
    active_neighbors
}

fn determine_new_state_3d(point: &Point, map: &HashMap<Point, bool>) -> bool {
    match map.get(point) {
        Some(v) => match *v {
            true => {
                matches!(count_active_neighbors_3d(point, map), 2..=3)
            }
            false => {
                matches!(count_active_neighbors_3d(point, map), 3)
            }
        },
        None => matches!(count_active_neighbors_3d(point, map), 3),
    }
}

fn count_active_neighbors_4d(point: &Point4D, map: &HashMap<Point4D, bool>) -> i64 {
    let mut active_neighbors: i64 = 0;
    for x in [-1, 0, 1 as i64] {
        for y in [-1, 0, 1 as i64] {
            for z in [-1, 0, 1 as i64] {
                for w in [-1, 0, 1 as i64] {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    let this_neighbor = Point4D {
                        x: point.x + x,
                        y: point.y + y,
                        z: point.z + z,
                        w: point.w + w,
                    };
                    if let Some(&v) = map.get(&this_neighbor) {
                        active_neighbors += v as i64;
                    };
                }
            }
        }
    }
    active_neighbors
}

fn determine_new_state_4d(point: &Point4D, map: &HashMap<Point4D, bool>) -> bool {
    match map.get(point) {
        Some(v) => match *v {
            true => {
                matches!(count_active_neighbors_4d(point, map), 2..=3)
            }
            false => {
                matches!(count_active_neighbors_4d(point, map), 3)
            }
        },
        None => matches!(count_active_neighbors_4d(point, map), 3),
    }
}
