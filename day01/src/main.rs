use std::collections::HashSet;

fn part1(input: &str) {
    let mut set = HashSet::new();
    for line in input.lines() {
        let n = line.parse::<i32>().unwrap();
        if set.contains(&(2020 - n)) {
            println!("Part 1: {}", n * (2020 - n));
            break;
        }
        set.insert(n);
    }
}

fn part2(input: &str) {
    let mut set = HashSet::new();
    'outer: for i in input.lines() {
        let x = i.parse::<i32>().unwrap();
        for j in input.lines() {
            let y = j.parse::<i32>().unwrap();
            let z = 2020 - x - y;
            if set.contains(&z) {
                println!("Part 2: {}", x * y * (2020 - x - y));
                break 'outer;
            }
            set.insert(x);
        }
    }
}

fn main() {
    let input = include_str!("day01.txt");
    part1(input);
    part2(input);
}
