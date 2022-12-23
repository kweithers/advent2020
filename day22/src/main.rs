use std::collections::{HashSet, VecDeque};

fn main() {
    let mut p1 = VecDeque::from([
        29, 25, 9, 1, 17, 28, 12, 49, 8, 15, 41, 31, 39, 24, 40, 23, 6, 21, 13, 45, 20, 2, 42, 47,
        10,
    ]);
    let mut p2 = VecDeque::from([
        46, 27, 44, 18, 30, 50, 37, 11, 43, 35, 34, 4, 22, 7, 33, 16, 36, 26, 48, 19, 38, 14, 5, 3,
        32,
    ]);

    while p1.len() > 0 && p2.len() > 0 {
        play_round(&mut p1, &mut p2);
    }

    let mut score: i32 = p2
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) as i32 * n)
        .sum();
    println!("Part 1: {}", score);

    // Part 2
    // Reset the decks
    p1 = VecDeque::from([
        29, 25, 9, 1, 17, 28, 12, 49, 8, 15, 41, 31, 39, 24, 40, 23, 6, 21, 13, 45, 20, 2, 42, 47,
        10,
    ]);
    p2 = VecDeque::from([
        46, 27, 44, 18, 30, 50, 37, 11, 43, 35, 34, 4, 22, 7, 33, 16, 36, 26, 48, 19, 38, 14, 5, 3,
        32,
    ]);

    if play_game(&mut p1, &mut p2) {
        score = p1
            .iter()
            .rev()
            .enumerate()
            .map(|(i, n)| (i + 1) as i32 * n)
            .sum();
    } else {
        score = p2
            .iter()
            .rev()
            .enumerate()
            .map(|(i, n)| (i + 1) as i32 * n)
            .sum();
    }
    println!("Part 2: {}", score);
}

fn play_round(p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>) {
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();

    if c1 > c2 {
        p1.push_back(c1);
        p1.push_back(c2);
    } else {
        p2.push_back(c2);
        p2.push_back(c1);
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct GameState {
    p1: VecDeque<i32>,
    p2: VecDeque<i32>,
}

fn play_game(p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>) -> bool /* true = Player 1 wins */ {
    let mut prev_states: HashSet<GameState> = HashSet::new();

    while p1.len() > 0 && p2.len() > 0 {
        if prev_states.contains(&GameState {
            p1: p1.clone(),
            p2: p2.clone(),
        }) {
            return true;
        }
        prev_states.insert(GameState {
            p1: p1.clone(),
            p2: p2.clone(),
        });

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        // Recursive Game
        if p1.len() as i32 >= c1 && p2.len() as i32 >= c2 {
            let mut deck1 = VecDeque::new();
            for (i, n) in p1.iter().enumerate() {
                if i == c1 as usize {
                    break;
                }
                deck1.push_back(*n);
            }
            let mut deck2 = VecDeque::new();
            for (i, n) in p2.iter().enumerate() {
                if i == c2 as usize {
                    break;
                }
                deck2.push_back(*n);
            }

            if play_game(&mut deck1, &mut deck2) {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
            continue;
        }

        // Regular Round
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    p1.len() > 0
}
