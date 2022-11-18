fn main() {
    let input = include_str!("day13.txt");
    let mut lines = input.lines();
    let start_time = lines.next().unwrap().parse::<i64>().unwrap();

    let mut buses = Vec::new();
    let bus_schedule = lines.next().unwrap().split(',');
    for (i, bus) in bus_schedule.enumerate() {
        if let Ok(bus) = bus.parse::<i64>() {
            buses.push((i as i64, bus));
        }
    }

    let mut best_bus_id = 0;
    let mut best_bus_time = i64::MAX;
    for (_, id) in buses.iter() {
        let next_bus_time = ((start_time / id) + 1) * id;
        if next_bus_time < best_bus_time {
            best_bus_id = *id;
            best_bus_time = next_bus_time;
        }
    }
    println!("Part 1: {}", best_bus_id * (best_bus_time - start_time));

    // Use Chinese remainder theorem to solve series of congruences
    // See scratch.txt
    let crt: usize = 905694340256752;
    println!("Part 2: {}", crt);
}
