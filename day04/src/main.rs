use std::collections::HashMap;
use std::ops::RangeInclusive;

struct Passport {
    byr: usize,  // (Birth Year) 1920
    iyr: usize,  // (Issue Year) 2010
    eyr: usize,  // (Expiration Year) 2029
    hgt: String, // (Height) 181cm
    hcl: String, // (Hair Color) #6b5442
    ecl: String, // (Eye Color) gry
    pid: String, // (Passport ID) 591597745
                 // cid: usize,  // (Country ID) 123; Removing since this is optional
}

fn hashmap_to_passport(m: &HashMap<String, String>) -> Passport {
    Passport {
        byr: m.get("byr").unwrap().parse::<usize>().unwrap(),
        iyr: m.get("iyr").unwrap().parse::<usize>().unwrap(),
        eyr: m.get("eyr").unwrap().parse::<usize>().unwrap(),
        hgt: m.get("hgt").unwrap().to_string(),
        hcl: m.get("hcl").unwrap().to_string(),
        ecl: m.get("ecl").unwrap().to_string(),
        pid: m.get("pid").unwrap().to_string(),
    }
}

fn main() {
    let input = include_str!("day04.txt");
    let lines = input.lines();

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut current_passport: HashMap<String, String> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            passports.push(current_passport);
            current_passport = HashMap::new();
            continue;
        }
        parse_line(line, &mut current_passport);
    }

    let mut valid_passports = Vec::new();
    for passport in passports.iter() {
        if passport.is_valid() {
            valid_passports.push(passport);
        }
    }
    println!("Part 1: {}", valid_passports.len());
    println!(
        "Part 2: {}",
        valid_passports
            .iter()
            .map(|x| hashmap_to_passport(x).is_valid())
            .filter(|b| *b == true)
            .count()
    )
}

fn parse_line(line: &str, current_passport: &mut HashMap<String, String>) {
    let tokens = line.split(' ');
    for token in tokens {
        let mut kv = token.split(':');
        let key = kv.next().unwrap().to_string();
        let value = kv.next().unwrap().to_string();
        current_passport.insert(key, value);
    }
}

trait Valid {
    fn is_valid(&self) -> bool;
}

impl Valid for HashMap<String, String> {
    fn is_valid(&self) -> bool {
        self.contains_key("byr")
            && self.contains_key("iyr")
            && self.contains_key("eyr")
            && self.contains_key("hgt")
            && self.contains_key("hcl")
            && self.contains_key("ecl")
            && self.contains_key("pid")
    }
}

impl Valid for Passport {
    fn is_valid(&self) -> bool {
        RangeInclusive::new(1920, 2002).contains(&self.byr)
            && RangeInclusive::new(2010, 2020).contains(&self.iyr)
            && RangeInclusive::new(2020, 2030).contains(&self.eyr)
            && is_valid_height(&self.hgt)
            && is_valid_hair_color(&self.hcl)
            && is_valid_eye_color(&self.ecl)
            && is_valid_pid(&self.pid)
    }
}

fn is_valid_height(s: &String) -> bool {
    if s.ends_with("in") {
        return RangeInclusive::new(59, 76)
            .contains(&(s.strip_suffix("in").unwrap().parse::<usize>().unwrap()));
    }
    if s.ends_with("cm") {
        return RangeInclusive::new(150, 193)
            .contains(&(s.strip_suffix("cm").unwrap().parse::<usize>().unwrap()));
    }
    false
}

fn is_valid_hair_color(s: &String) -> bool {
    let hex = s.get(1..).unwrap().chars();
    let valid_hex = hex
        .map(|c| c.is_ascii_hexdigit())
        .filter(|&b| b == true)
        .count()
        == 6;
    s.starts_with('#') && valid_hex
}

fn is_valid_eye_color(s: &String) -> bool {
    s.eq("amb")
        || s.eq("blu")
        || s.eq("brn")
        || s.eq("gry")
        || s.eq("grn")
        || s.eq("hzl")
        || s.eq("oth")
}

fn is_valid_pid(s: &String) -> bool {
    let valid_digits = s
        .chars()
        .map(|c| c.is_ascii_digit())
        .filter(|&b| b == true)
        .count()
        == 9;
    s.len() == 9 && valid_digits
}
