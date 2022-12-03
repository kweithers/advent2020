use std::collections::{HashSet,HashMap};
use regex::Regex;
fn main() {
    let rinput = include_str!("rules.txt");
    let minput = include_str!("messages.txt");

    let mut map: HashMap<String, String> = HashMap::new(); // (Rule #, Regex)
    let mut keys = HashSet::new();
    let mut solved_keys = HashSet::new();

    let _ : () = rinput.lines().map(|l|create_rule(l, &mut map, &mut keys)).collect();
    map.insert("123".to_owned(),"a".to_owned());
    map.insert("97".to_owned(),"b".to_owned());    
    solved_keys.insert("123".to_owned());
    solved_keys.insert("97".to_owned());

    while keys.contains("0") {
        for k in keys.clone().iter() {
            simplify_rule(k,&mut map, &mut solved_keys, &mut keys);
        }
    }
    // Add ^ to beginning and $ to end of final regex
    let zero  =  map.get("0").unwrap().replace("[","").replace("]","");
    let wrapped = "^".to_owned() + zero.as_str() + "$";
    let re = Regex::new(wrapped.as_str()).unwrap();
    
    println!{"Part 1: {}",minput.lines().filter(|l| re.is_match(l)).count()};

}

fn create_rule(s: &str, map: &mut HashMap<String, String>, keys: &mut HashSet<String>) {
    let mut splt = s.split(": ");
    let key = splt.next().unwrap().to_owned();
    
    let rule: String = splt.next().unwrap().split(" ").map(|s| "[".to_owned() + s + "]").collect();
    let final_rule: String = "(".to_owned() + rule.as_str() + ")";
    map.insert(key.clone(),final_rule);
    keys.insert(key);
}

fn simplify_rule(key: &String, map: &mut HashMap<String, String>, solved_keys: &mut HashSet<String>, keys: &mut HashSet<String>) {
    let mut val = map.get(key).unwrap().clone();
    for k in solved_keys.iter() {
        let key = "[".to_owned() + k + "]";
        val = val.replace(&key,map.get(k).unwrap().as_str());
    }

    if is_done(&val) {
        solved_keys.insert(key.clone());
        keys.remove(key);
    }
    map.insert(key.clone(), val);
}

fn is_done(val: &String) -> bool {
    for c in val.chars() {
        if c.is_numeric() {
            return false
        }
    }
    true
}