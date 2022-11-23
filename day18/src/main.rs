use std::collections::VecDeque;

fn main() {
    let input = include_str!("day18.txt");
    let s1: usize = input.lines().map(|s| evaluate_expression(s, true)).sum();
    println!("Part 1: {}", s1);
    let s2: usize = input.lines().map(|s| evaluate_expression(s, false)).sum();
    println!("Part 2: {}", s2);
}

fn evaluate_expression(s: &str, part1: bool) -> usize {
    let open_paren = s.find('(');
    if let Some(open_paren_index) = open_paren {
        let close_paren_index =
            find_close_paren_index(&s[open_paren_index + 1..]) + open_paren_index + 1;

        let substring = &s[open_paren_index + 1..close_paren_index];
        let substring_value = evaluate_expression(substring, part1);

        let mut new_string = s.to_string();
        new_string.replace_range(
            open_paren_index..=close_paren_index,
            substring_value.to_string().as_str(),
        );
        return evaluate_expression(new_string.as_str(), part1);
    }

    let (mut nums, mut operations) = {
        let mut nums = VecDeque::new();
        let mut operations = VecDeque::new();
        for (i, token) in s.split(" ").enumerate() {
            match i % 2 {
                0 => nums.push_back(token.parse::<usize>().unwrap()),
                1 => operations.push_back(token),
                _ => unreachable!(),
            }
        }
        (nums, operations)
    };

    while nums.len() > 1 {
        if part1 {
            // Do operations left-to-right
            let x = nums.pop_front().unwrap();
            let y = nums.pop_front().unwrap();
            let op = operations.pop_front().unwrap();
            nums.push_front(evaluate_operation(x, y, op));
        } else {
            // Do all additions first
            if let Some(addition_index) = operations.iter().position(|&c| c == "+") {
                let x = nums.remove(addition_index).unwrap();
                let y = nums.remove(addition_index).unwrap();
                let op = operations.remove(addition_index).unwrap();
                nums.insert(addition_index, evaluate_operation(x, y, op));
            } else {
                let x = nums.pop_front().unwrap();
                let y = nums.pop_front().unwrap();
                let op = operations.pop_front().unwrap();
                nums.push_front(evaluate_operation(x, y, op));
            }
        }
    }
    nums[0]
}

fn evaluate_operation(x: usize, y: usize, op: &str) -> usize {
    match op {
        "+" => x + y,
        "*" => x * y,
        _ => unreachable!(),
    }
}

fn find_close_paren_index(s: &str) -> usize {
    let mut n_open = 0;
    for (index, token) in s.chars().enumerate() {
        match token {
            '(' => n_open += 1,
            ')' => match n_open {
                0 => return index,
                _ => n_open -= 1,
            },
            _ => (),
        }
    }
    unreachable!()
}
