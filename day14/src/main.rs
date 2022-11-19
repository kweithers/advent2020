use std::collections::HashMap;

fn main() {
    let input = include_str!("day14.txt");
    let mut mem: HashMap<i64, i64> = HashMap::new();
    let mut current_mask = [' '; 36];

    for line in input.lines() {
        parse_line(line, &mut current_mask, &mut mem);
    }
    let s: i64 = mem.values().sum();
    println!("Part 1: {}", s);

    //Part 2
    mem = HashMap::new();
    current_mask = [' '; 36];

    for line in input.lines() {
        parse_line2(line, &mut current_mask, &mut mem);
    }
    let s2: i64 = mem.values().sum();
    println!("Part 2: {}", s2);
}

fn parse_line(s: &str, current_mask: &mut [char; 36], mem: &mut HashMap<i64, i64>) {
    if s.starts_with("mask") {
        set_mask(s, current_mask);
    } else if s.starts_with("mem") {
        set_mem_value(s, *current_mask, mem);
    }
}

fn set_mask(s: &str, current_mask: &mut [char; 36]) {
    let mask = s.strip_prefix("mask = ").unwrap();
    for (i, c) in mask.chars().enumerate() {
        current_mask[i] = c;
    }
}

fn set_mem_value(s: &str, current_mask: [char; 36], mem: &mut HashMap<i64, i64>) {
    let mut tokens = s.strip_prefix("mem[").unwrap().split("] = ");
    let addr = tokens.next().unwrap().parse::<i64>().unwrap();
    let val = tokens.next().unwrap().parse::<i64>().unwrap();

    mem.insert(addr, apply_mask(val, current_mask));
}

fn apply_mask(val: i64, current_mask: [char; 36]) -> i64 {
    let val_bits = format!("{:036b}", val); //zero-pad to 36bits

    let mut result = String::new();
    for (i, bit) in val_bits.chars().enumerate() {
        match current_mask[i] {
            '0' => result.push('0'),
            '1' => result.push('1'),
            'X' => result.push(bit),
            _ => unreachable!(),
        }
    }
    i64::from_str_radix(result.as_str(), 2).unwrap()
}

fn parse_line2(s: &str, current_mask: &mut [char; 36], mem: &mut HashMap<i64, i64>) {
    if s.starts_with("mask") {
        set_mask(s, current_mask);
    } else if s.starts_with("mem") {
        set_many_mem_values(s, *current_mask, mem);
    }
}

fn set_many_mem_values(s: &str, current_mask: [char; 36], mem: &mut HashMap<i64, i64>) {
    let mut tokens = s.strip_prefix("mem[").unwrap().split("] = ");
    let addr = tokens.next().unwrap().parse::<i64>().unwrap();
    let val = tokens.next().unwrap().parse::<i64>().unwrap();

    let addr_bits = format!("{:036b}", addr); //zero-pad to 36bits

    let mut addr_bits_after_mask = String::new();
    for (i, bit) in current_mask.iter().enumerate() {
        match bit {
            // apply 0s and 1s from bitmask
            '0' => addr_bits_after_mask.push(addr_bits.chars().nth(i).unwrap()),
            '1' => addr_bits_after_mask.push('1'),
            'X' => addr_bits_after_mask.push('X'),
            _ => unreachable!(),
        }
    }

    set_mem_values_with_floaters(addr_bits_after_mask, val, current_mask, mem);
}

fn set_mem_values_with_floaters(
    addr_bits: String,
    val: i64,
    current_mask: [char; 36],
    mem: &mut HashMap<i64, i64>,
) {
    if addr_bits.contains('X') {
        let x_id = addr_bits.find('X').unwrap();

        let mut float_on = addr_bits.clone();
        float_on.remove(x_id);
        float_on.insert(x_id, '1');

        let mut float_off = addr_bits.clone();
        float_off.remove(x_id);
        float_off.insert(x_id, '0');

        set_mem_values_with_floaters(float_on, val, current_mask, mem);
        set_mem_values_with_floaters(float_off, val, current_mask, mem);
    } else {
        let addr = i64::from_str_radix(addr_bits.as_str(), 2).unwrap();
        mem.insert(addr, val);
    }
}
