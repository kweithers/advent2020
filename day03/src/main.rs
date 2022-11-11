const LINE_LENGTH: usize = 31;

struct Slope {
    right: usize,
    down: usize,
}

fn process_line(line: &str, x_val: usize) -> bool {
    line.chars().nth(x_val).unwrap() == '#'
}

fn traverse_map(input: &str, slope: Slope) -> usize {
    let lines = input.lines();
    let mut trees = 0;
    let mut x_val = 0;
    for (i, line) in lines.into_iter().enumerate() {
        if i % slope.down == 0 {
            trees += process_line(line, x_val) as usize;
            x_val += slope.right;
            x_val %= LINE_LENGTH;
        }
    }
    trees
}

fn part1(input: &str) {
    let trees = traverse_map(input, Slope { right: 3, down: 1 });
    println!("Part 1: {}", trees);
}

fn part2(input: &str) {
    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let mut totals = Vec::new();
    for slope in slopes {
        totals.push(traverse_map(input, slope));
    }
    println!("Part 2: {}", totals.into_iter().product::<usize>());
}

fn main() {
    let input = include_str!("day03.txt");
    part1(input);
    part2(input);
}
