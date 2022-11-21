use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

struct Rule {
    name: String,
    first_range: RangeInclusive<usize>,
    second_range: RangeInclusive<usize>,
}

fn main() {
    let rules = include_str!("day16.txt")
        .lines()
        .map(|line| parse_rule(line))
        .collect::<Vec<Rule>>();
    let my_ticket: Vec<usize> = vec![
        137, 173, 167, 139, 73, 67, 61, 179, 103, 113, 163, 71, 97, 101, 109, 59, 131, 127, 107, 53,
    ];
    let nearby_tickets = include_str!("nearby_tickets.txt")
        .lines()
        .map(|x| {
            x.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let error_rate: usize = nearby_tickets
        .iter()
        .map(|t| get_invalid_params(&t, &rules))
        .sum();
    println!("Part 1: {}", error_rate);

    let valid_tickets = nearby_tickets
        .iter()
        .filter(|t| get_invalid_params(&t, &rules) == 0)
        .collect::<Vec<&Vec<usize>>>();

    let mut name_indices_map: HashMap<String, HashSet<usize>> = HashMap::new();
    for rule in rules {
        get_valid_indices_for_rule(&rule, &valid_tickets, &mut name_indices_map);
    }

    let mut used_indices: HashSet<usize> = HashSet::new();
    let mut final_assignments: HashMap<String, usize> = HashMap::new();
    loop {
        for (name, v) in name_indices_map.iter() {
            let mut d = v.difference(&used_indices);
            if v.difference(&used_indices).count() == 1 {
                let index = d.next().unwrap();
                final_assignments.insert(name.to_string(), *index);
                used_indices.insert(*index);
            }
        }
        if final_assignments.contains_key("departure location")
            && final_assignments.contains_key("departure station")
            && final_assignments.contains_key("departure platform")
            && final_assignments.contains_key("departure track")
            && final_assignments.contains_key("departure date")
            && final_assignments.contains_key("departure time")
        {
            break;
        }
    }
    println!(
        "Part 2: {}",
        my_ticket[*final_assignments.get("departure location").unwrap()]
            * my_ticket[*final_assignments.get("departure station").unwrap()]
            * my_ticket[*final_assignments.get("departure platform").unwrap()]
            * my_ticket[*final_assignments.get("departure track").unwrap()]
            * my_ticket[*final_assignments.get("departure date").unwrap()]
            * my_ticket[*final_assignments.get("departure time").unwrap()]
    );
}

fn parse_rule(line: &str) -> Rule {
    let mut tokens = line.split(": ");
    let name = tokens.next().unwrap().to_string();
    let mut ranges = tokens.next().unwrap().split(" or ");

    let mut first_range_tokens = ranges.next().unwrap().split("-");
    let first_range = RangeInclusive::new(
        first_range_tokens.next().unwrap().parse::<usize>().unwrap(),
        first_range_tokens.next().unwrap().parse::<usize>().unwrap(),
    );

    let mut second_range_tokens = ranges.next().unwrap().split("-");
    let second_range = RangeInclusive::new(
        second_range_tokens
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        second_range_tokens
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );

    Rule {
        name,
        first_range,
        second_range,
    }
}

fn get_invalid_params(ticket: &Vec<usize>, rules: &Vec<Rule>) -> usize {
    let mut invalids = 0;
    for value in ticket {
        let mut result = false;
        for rule in rules {
            result =
                result || (rule.first_range.contains(value) || rule.second_range.contains(value));
        }
        if !result {
            invalids += *value;
        }
    }
    invalids
}

fn get_valid_indices_for_rule(
    rule: &Rule,
    valid_tickets: &Vec<&Vec<usize>>,
    map: &mut HashMap<String, HashSet<usize>>,
) {
    let mut counts = [0usize; 20];
    for ticket in valid_tickets {
        for (index, value) in ticket.iter().enumerate() {
            if rule.first_range.contains(value) || rule.second_range.contains(value) {
                counts[index] += 1;
            }
        }
    }
    let mut valid_indicies = HashSet::new();
    for (i, v) in counts.iter().enumerate() {
        if *v == valid_tickets.len() {
            valid_indicies.insert(i);
        }
    }
    map.insert(rule.name.to_string(), valid_indicies);
}
