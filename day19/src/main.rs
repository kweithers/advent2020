use regex::Regex;
use std::collections::{HashMap, HashSet};
fn main() {
    let rinput = include_str!("rules2.txt");
    let minput = include_str!("messages.txt");
    let mut good_messages: HashSet<String> = HashSet::new();

    let mut map: HashMap<String, String> = HashMap::new(); // (Rule #, Regex)
    let mut keys = HashSet::new();
    let mut solved_keys = HashSet::new();

    let _: () = rinput
        .lines()
        .map(|l| create_rule(l, &mut map, &mut keys))
        .collect();

    map.insert("123".to_owned(), "a".to_owned());
    map.insert("97".to_owned(), "b".to_owned());
    solved_keys.insert("123".to_owned());
    solved_keys.insert("97".to_owned());

    while keys.contains("0") {
        for k in keys.clone().iter() {
            simplify_rule(k, &mut map, &mut solved_keys, &mut keys);
        }
    }
    // Remove the 'boxes' [] we put the keys in to ensure uniqueness (i.e. 23 would be impacted by replace(23) and replace(123))
    let zero = map.get("0").unwrap().replace("[", "").replace("]", "");
    // Add ^ to beginning and $ to end of final regex
    let wrapped = "^".to_owned() + zero.as_str() + "$";

    // Use many different values of i until the length of the regex gets longer than our longest message
    for i in 1..5 {
        let this_regex = wrapped.replace("i", &i.to_string());
        let re = Regex::new(this_regex.as_str()).unwrap();
        let good: Vec<&str> = minput.lines().filter(|l| re.is_match(l)).collect();
        let _: Vec<bool> = good
            .iter()
            .map(|&x| good_messages.insert(x.to_owned()))
            .collect();
    }
    println!("{}", good_messages.len())
}

fn create_rule(s: &str, map: &mut HashMap<String, String>, keys: &mut HashSet<String>) {
    let mut splt = s.split(": ");
    let key = splt.next().unwrap().to_owned();

    let rule: String = splt
        .next()
        .unwrap()
        .split(" ")
        .map(|s| "[".to_owned() + s + "]")
        .collect();
    let final_rule: String = "(".to_owned() + rule.as_str() + ")";
    map.insert(key.clone(), final_rule);
    keys.insert(key);
}

fn simplify_rule(
    key: &String,
    map: &mut HashMap<String, String>,
    solved_keys: &mut HashSet<String>,
    keys: &mut HashSet<String>,
) {
    let mut val = map.get(key).unwrap().clone();
    for k in solved_keys.iter() {
        let key = "[".to_owned() + k + "]";
        val = val.replace(&key, map.get(k).unwrap().as_str());
        // try to replace the special bois
        // i
        let key = "[(".to_owned() + k + "){i}]";
        let raw_val = map.get(k).unwrap().as_str();
        let new_val = "(".to_owned() + raw_val + "){i}";
        val = val.replace(&key, new_val.as_str());
        // +
        let key = "[(".to_owned() + k + ")+]";
        let raw_val = map.get(k).unwrap().as_str();
        let new_val = "(".to_owned() + raw_val + ")+";
        val = val.replace(&key, new_val.as_str());
    }

    if is_done(&val) {
        solved_keys.insert(key.clone());
        keys.remove(key);
    }
    map.insert(key.clone(), val);
}

fn is_done(val: &String) -> bool {
    let mut before_iter = val.split("{");
    let before = before_iter.next().unwrap().to_owned();

    for c in before.as_str().chars() {
        if c.is_numeric() {
            return false;
        }
    }

    let mut after_iter = val.split("}");
    let _ = after_iter.next();
    let after = after_iter.next();

    match after {
        Some(str) => {
            for c in str.chars() {
                if c == '{' {
                    break;
                }
                if c.is_numeric() {
                    return false;
                }
            }
        }
        None => (),
    }
    true
}
