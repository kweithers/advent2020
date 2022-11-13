use std::collections::HashSet;
// use std::collections::HashMap;
use counter::Counter;

fn main() {
    let input = include_str!("day06.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    
}

fn part1(input: &str) -> usize {
    let lines = input.lines();

    let mut groups: Vec<HashSet<String>> = Vec::new();
    let mut current_group: HashSet<String> = HashSet::new();
    for line in lines {
        if line.is_empty() {
            groups.push(current_group);
            current_group = HashSet::new();
            continue;
        }
        parse_line_part1(line, &mut current_group);
    }

    let result: usize = groups.iter().map(|group| group.len()).sum();
    result
}

fn parse_line_part1(line: &str, current_group: &mut HashSet<String>) {
    for token in line.chars() {
        current_group.insert(token.to_string());
    }
}

fn part2(input: &str) -> usize {
    let lines = input.lines();

    let mut groups = Vec::new();
    let mut sizes = Vec::new();
    let mut current_group = "".chars().collect::<Counter<_>>();
    let mut current_group_size: usize = 0;

    for line in lines {
        if line.is_empty() {
            groups.push(current_group);
            sizes.push(current_group_size);
            current_group = "".chars().collect::<Counter<_>>();
            current_group_size = 0;
            continue;
        }
        current_group_size+=1;
        parse_line_part2(line, &mut current_group);
    }

    let mut part2 = 0;
    for i in 0..groups.len() {
        part2 += count_all_yes(&groups[i], &sizes[i]);
    }
    part2
}

fn parse_line_part2(line: &str, current_group: &mut Counter<char, usize>) {
    for token in line.chars() {
        current_group[&token]+=1;
    }
}

fn count_all_yes(group: &Counter<char, usize>, group_size: &usize) -> usize {
    let mut result = 0;
    for (_, n) in group {
        if n == group_size {
            result+=1;
        }
    }
    result
}