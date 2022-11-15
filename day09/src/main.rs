use std::collections::HashSet;
fn main() {
    let input = include_str!("day09.txt");
    let mut nums = [0usize; 1000];
    for (i, line) in input.lines().enumerate() {
        nums[i] = line.parse::<usize>().unwrap();
    }
    let mut sets = Vec::new();
    for _ in 0..1000 {
        sets.push(HashSet::<usize>::new());
    }

    let mut target = 0;
    for i in 0..1000 {
        // If outside the preamble, check if the number is valid.
        if i >= 25 {
            if !sets[i].contains(&nums[i]) {
                target = nums[i];
                println!("Part 1: {}", nums[i]);
                break;
            }
        }
        for j in i + 1..std::cmp::min(1000, i + 26) {
            // If the numbers are the same, skip.
            if nums[i] == nums[j] {
                continue;
            }
            // Insert into the next 25 sets.
            for k in i + 1..std::cmp::min(1000, i + 26) {
                sets[k].insert(nums[i] + nums[j]);
            }
        }
    }

    // Part 2
    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    loop {
        let slice: &[usize] = &nums[left_index..right_index + 1];
        let sum: usize = slice.iter().sum();
        if sum == target {
            let (min, max) = { (slice.iter().min().unwrap(), slice.iter().max().unwrap()) };
            println!("Part 2: {}", min + max);
            break;
        } else if sum < target {
            right_index += 1;
        } else {
            left_index += 1;
        }
    }
}
