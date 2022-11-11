use std::ops::RangeInclusive;

struct PasswordValidator {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordValidator {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .copied()
                .filter(|&b| b == self.byte)
                .count(),
        )
    }
}

fn parse_line(s: &str) -> (PasswordValidator, &str) {
    let mut tokens = s.split(':');
    let policy = tokens.next().unwrap();
    let password = tokens.next().unwrap();

    tokens = policy.split(' ');
    let range = tokens.next().unwrap();
    let byte = tokens.next().unwrap().as_bytes()[0];

    tokens = range.split('-');
    let min = tokens.next().unwrap();
    let max = tokens.next().unwrap();

    let range = (min.parse::<usize>().unwrap())..=(max.parse::<usize>().unwrap());

    (PasswordValidator { byte, range }, password)
}

struct PasswordValidatorPart2 {
    byte: u8,
    positions: [usize; 2],
}

impl PasswordValidatorPart2 {
    fn is_valid(&self, password: &str) -> bool {
        let mut iter = password.bytes().enumerate();
        let mut n = 0;

        while let Some(x) = iter.next() {
            if self.positions.contains(&x.0) {
                if x.1 == self.byte {
                    n += 1;
                }
            }
        }
        n == 1
    }
}

fn parse_line_part2(s: &str) -> (PasswordValidatorPart2, &str) {
    let mut tokens = s.split(':');
    let policy = tokens.next().unwrap();
    let password = tokens.next().unwrap();

    tokens = policy.split(' ');
    let range = tokens.next().unwrap();
    let byte = tokens.next().unwrap().as_bytes()[0];

    tokens = range.split('-');
    let min = tokens.next().unwrap().parse::<usize>().unwrap();
    let max = tokens.next().unwrap().parse::<usize>().unwrap();

    (
        PasswordValidatorPart2 {
            byte,
            positions: [min, max],
        },
        password,
    )
}

fn part1(input: &str) {
    println!(
        "Part 1: {}",
        input
            .lines()
            .map(parse_line)
            .map(|(policy, password)| policy.is_valid(password))
            .filter(|b| *b == true)
            .count()
    );
}

fn part2(input: &str) {
    println!(
        "Part 2: {}",
        input
            .lines()
            .map(parse_line_part2)
            .map(|(policy, password)| policy.is_valid(password))
            .filter(|b| *b == true)
            .count()
    );
}

fn main() {
    let input = include_str!("day02.txt");
    part1(input);
    part2(input);
}
