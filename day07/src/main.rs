use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    color: String,
    connections: Vec<Connection>,
}

#[derive(Debug)]
struct Connection {
    color: String,
    weight: usize,
}

fn main() {
    let input = include_str!("day07.txt");
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (key, value) = parse_line(line);
        nodes.insert(key, value);
    }
    println!("Part 1: {}", count_colors(&nodes));
    println!(
        "Part 2: {}",
        count_bags(&nodes, nodes.get("shiny gold").unwrap())
    );
}

fn parse_line(line: &str) -> (String, Node) {
    let (color, connections) = {
        let mut tokens = line.split(" bags contain ");
        (
            tokens.next().unwrap().to_string(),
            parse_connections(tokens.next().unwrap()),
        )
    };
    (color.clone(), Node { color, connections })
}

fn parse_connections(line: &str) -> Vec<Connection> {
    let tokens = line.split(", ");
    let mut connections = Vec::new();
    for token in tokens {
        let c = parse_single_connection(token);
        if let Some(connection) = c {
            connections.push(connection)
        }
    }
    connections
}

fn parse_single_connection(s: &str) -> Option<Connection> {
    let mut tokens = s.split(" ");
    let first_token = tokens.next().unwrap();
    if first_token == "no" {
        return None;
    }
    let weight = first_token.parse::<usize>().unwrap();
    let mut color = tokens.next().unwrap().to_string();
    color.push_str(" ");
    color.push_str(tokens.next().unwrap());
    Some(Connection { color, weight })
}

fn count_colors(nodes: &HashMap<String, Node>) -> usize {
    let mut color_set: HashSet<String> = HashSet::new();
    for bag in nodes.keys() {
        if bag == "shiny gold" {
            continue;
        }
        if node_contains_shiny_gold(&nodes, nodes.get(bag).unwrap()) {
            color_set.insert(bag.to_string());
        }
    }
    color_set.len()
}

fn node_contains_shiny_gold(map: &HashMap<String, Node>, n: &Node) -> bool {
    if n.color == "shiny gold" {
        return true;
    }
    let mut results = Vec::new();
    for connection in &n.connections {
        results.push(node_contains_shiny_gold(
            map,
            map.get(&connection.color).unwrap(),
        ));
    }
    results.contains(&true)
}

fn count_bags(map: &HashMap<String, Node>, n: &Node) -> usize {
    let mut total = 0;
    for connection in &n.connections {
        total += connection.weight;
        total += connection.weight * count_bags(map, map.get(&connection.color).unwrap());
    }
    total
}
