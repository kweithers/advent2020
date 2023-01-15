fn main() {
    // Puzzle input: 523764819
    // cups[cup] = next_cup
    // cup 0 does not exist: set to -1

    let mut cups = vec![-1, 9, 3, 7, 8, 2, 4, 6, 1, 5];
    let mut current_cup = 5;

    for _rounds in 0..100 {
        current_cup = next_round(current_cup, &mut cups, 9);
    }
    println!("Part 1: {:?}", cups);

    // Part 2
    let mut cups = vec![-1, 9, 3, 7, 8, 2, 4, 6, 1, 5];
    let mut current_cup = 5;

    // Last cup points to new 10 cup, instead of wrapping around
    cups[9] = 10;
    for v in 11..=1000000 {
        cups.push(v)
    }
    // The millionth cup one points to 5, the first element
    cups.push(5);

    for _rounds in 0..10000000 {
        current_cup = next_round(current_cup, &mut cups, 1000000);
    }

    let c1 = cups[1] as i64;
    let c2 = cups[c1 as usize] as i64;
    println!("Part 2: {}", c1 * c2);
}

fn next_round(current_cup: i32, cups: &mut Vec<i32>, n: i32) -> i32 {
    // Pick up the next three cups
    let c1 = cups[current_cup as usize];
    let c2 = cups[c1 as usize];
    let c3 = cups[c2 as usize];

    // Find destination cup
    let mut dest = current_cup - 1;
    loop {
        if dest == 0 {
            dest = n;
        }
        if dest == c1 || dest == c2 || dest == c3 {
            dest -= 1;
            continue;
        } else {
            break;
        }
    }
    // Remember the destinations next cup
    let dest_next = cups[dest as usize];

    // After picking up the three, set current cup's next cup as the element after the third
    cups[current_cup as usize] = cups[c3 as usize];

    // Set the dest next cup to the first cup picked up
    cups[dest as usize] = c1;

    // Set the third cup's next cup to destinations previous next cup
    cups[c3 as usize] = dest_next;

    // Return the next cup
    cups[current_cup as usize]
}
