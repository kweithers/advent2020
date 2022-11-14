use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Instruction {
    operation: String,
    argument: isize,
}

fn main() {
    let input = include_str!("day08.txt");
    let lines = input.lines();
    let mut instructions = Vec::new();

    for line in lines {
        instructions.push(parse_line(line));
    }
    println!(
        "Part 1: {}",
        evaluate_until_repeat_instruction(&instructions)
    );

    for (i, instruction) in instructions.iter().enumerate() {
        let mut these_instructions = instructions.clone();

        let mut new_operation = "".to_string();
        match instruction.operation.as_str() {
            "nop" => new_operation.push_str("jmp"),
            "jmp" => new_operation.push_str("nop"),
            _ => continue,
        }

        let new_instruction = Instruction {
            operation: new_operation,
            argument: instruction.argument,
        };
        these_instructions.remove(i);
        these_instructions.insert(i, new_instruction);
        let (b, v) = completes_without_cycle(&these_instructions);
        if b {
            println!("Part 2: {}", v);
            break;
        }
    }
}

fn parse_line(line: &str) -> Instruction {
    let (operation, argument) = {
        let mut tokens = line.split(" ");
        (
            tokens.next().unwrap().to_string(),
            tokens.next().unwrap().parse::<isize>().unwrap(),
        )
    };
    Instruction {
        operation,
        argument,
    }
}

fn evaluate_until_repeat_instruction(instructions: &Vec<Instruction>) -> isize {
    let mut accumulator: isize = 0;
    let mut current_instruction: isize = 0;
    let mut evaluated_instructions: HashSet<isize> = HashSet::new();

    loop {
        if evaluated_instructions.contains(&current_instruction) {
            return accumulator;
        }
        evaluate_single_instruction(
            &mut accumulator,
            &mut current_instruction,
            instructions,
            &mut evaluated_instructions,
        )
    }
}

fn evaluate_single_instruction(
    accumulator: &mut isize,
    current_instruction: &mut isize,
    instructions: &Vec<Instruction>,
    evaluated_instructions: &mut HashSet<isize>,
) {
    evaluated_instructions.insert(*current_instruction);
    match instructions[*current_instruction as usize]
        .operation
        .as_str()
    {
        "nop" => {
            *current_instruction += 1;
        }
        "acc" => {
            *accumulator += instructions[*current_instruction as usize].argument;
            *current_instruction += 1;
        }
        "jmp" => {
            *current_instruction += instructions[*current_instruction as usize].argument;
        }
        _ => (),
    }
}

fn completes_without_cycle(instructions: &Vec<Instruction>) -> (bool, isize) {
    let mut accumulator: isize = 0;
    let mut current_instruction: isize = 0;
    let mut evaluated_instructions: HashSet<isize> = HashSet::new();

    loop {
        if evaluated_instructions.contains(&current_instruction) {
            return (false, accumulator);
        }
        if current_instruction == instructions.len() as isize {
            return (true, accumulator);
        }
        evaluate_single_instruction(
            &mut accumulator,
            &mut current_instruction,
            instructions,
            &mut evaluated_instructions,
        )
    }
}
