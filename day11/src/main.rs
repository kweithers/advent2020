const COLUMNS: usize = 90;
const ROWS: usize = 99;

fn main() {
    let input = include_str!("day11.txt");
    let mut grid = [[' '; COLUMNS]; ROWS];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }
    loop {
        let next_grid = create_next_grid(&grid);
        if grid == next_grid {
            let occupied_seats = grid
                .iter()
                .flat_map(|r| r.iter())
                .filter(|x| **x == '#')
                .count();
            println!("Result: {}", occupied_seats);
            break;
        } else {
            grid = next_grid;
        }
    }
}

fn create_next_grid(grid: &[[char; COLUMNS]; ROWS]) -> [[char; COLUMNS]; ROWS] {
    let mut next_grid = [[' '; COLUMNS]; ROWS];
    for y in 0..ROWS {
        for x in 0..COLUMNS {
            next_grid[y][x] = determine_next_value(&grid, y as isize, x as isize);
        }
    }
    next_grid
}

fn determine_next_value(grid: &[[char; COLUMNS]; ROWS], y: isize, x: isize) -> char {
    match grid[y as usize][x as usize] {
        // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        'L' => parse_empty_seat(grid, y, x),
        // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
        '#' => parse_occupied_seat(grid, y, x),
        // Otherwise, the seat's state does not change.
        _ => grid[y as usize][x as usize],
    }
}

fn parse_empty_seat(grid: &[[char; COLUMNS]; ROWS], y: isize, x: isize) -> char {
    if count_occupied_neighbors(grid, y, x) == 0 {
        return '#';
    }
    'L'
}

fn parse_occupied_seat(grid: &[[char; COLUMNS]; ROWS], y: isize, x: isize) -> char {
    if count_occupied_neighbors(grid, y, x) >= 5 {
        return 'L';
    }
    '#'
}

fn count_occupied_neighbors(grid: &[[char; COLUMNS]; ROWS], y: isize, x: isize) -> usize {
    let mut n_occupied = 0;
    for j in [-1, 0, 1] {
        for i in [-1, 0, 1] {
            if i == 0 && j == 0 {
                continue;
            }
            n_occupied += get_neighbor(grid, y + j, x + i, j, i)
        }
    }
    n_occupied
}

fn get_neighbor(grid: &[[char; COLUMNS]; ROWS], y: isize, x: isize, j: isize, i: isize) -> usize {
    if x < 0 || x > (COLUMNS - 1) as isize || y < 0 || y > (ROWS - 1) as isize {
        return 0;
    } else if grid[y as usize][x as usize] == '.' {
        return get_neighbor(grid, y + j, x + i, j, i);
    }
    (grid[y as usize][x as usize] == '#') as usize
}
