use std::collections::HashMap;
fn main() {
    let input = include_str!("day10.txt");
    let mut nums = Vec::new();
    nums.push(0);
    for line in input.lines() {
        nums.push(line.parse::<usize>().unwrap());
    }
    nums.push(nums.iter().max().unwrap() + 3);
    nums.sort();

    let mut ones = 0;
    let mut threes = 0;
    for i in 1..nums.len() {
        match nums[i] - nums[i-1] {
            1 => ones +=1,
            3 => threes +=1,
            _ => (),
        }
    }
    println!("Part 1: {}", ones*threes);

    //Part 2
    let mut required_nums = Vec::new();
    let mut optional_nums = Vec::new();

    required_nums.push(0);
    for i in 1..nums.len()-1 {
        if nums[i+1] - nums[i] == 3{
            required_nums.push(nums[i]);
            required_nums.push(nums[i+1]);
        } else if !required_nums.contains(&nums[i]) {
            optional_nums.push(nums[i]);
        }
    }
    required_nums.dedup();

    let mut m: HashMap<(usize,usize), usize> = HashMap::new(); 
    m.insert((0,0), 1); // 0 Choose 0
    m.insert((0,1), 2); // 1 Choose 0 + 1 Choose 1
    m.insert((0,2), 4); // 2 Choose 0 + 2 Choose 1 + 2 Choose 2
    m.insert((1,2), 3); // 2 Choose 1 + 2 Choose 2
    m.insert((1,3), 7); // 3 Choose 1 + 3 Choose 2 + 3 Choose 3

    let mut factors = Vec::new();
    for i_lower in 0..required_nums.len()-1 {
        let lower = required_nums[i_lower];
        let upper = required_nums[i_lower+1];
        let diff = upper - lower;
        let need = (diff > 3) as usize;
        let n_elements = optional_nums.iter().filter(|n| (*n > &lower) && (*n < &upper)).count();

        factors.push(*m.get(&(need,n_elements)).unwrap());
    }

    let count: usize = factors.iter().product();
    println!("Part 2: {}", count);
}

