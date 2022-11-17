struct Instruction {
    action: char,
    value: i64,
}

fn parse_instruction(s: &str) -> Instruction {
    let mut chars = s.chars();
    let (action, value_chars) = chars.next().map(|c| (c, chars.as_str())).unwrap();
    Instruction {
        action,
        value: value_chars.parse::<i64>().unwrap(),
    }
}

trait BoatyMcBoatFace {
    fn manhattan_distance(&self) -> i64;
    fn apply_instruction(&mut self, inst: &Instruction);
    fn go_forward(&mut self, value: i64);
    fn rotate_right(&mut self, inst: &Instruction);
    fn rotate_left(&mut self, inst: &Instruction);
}

struct Ferry {
    direction: char,
    north_south: i64, //north is negative, south is positive
    east_west: i64,   //east is negative, west is positive
}

impl BoatyMcBoatFace for Ferry {
    fn manhattan_distance(&self) -> i64 {
        self.north_south.abs() + self.east_west.abs()
    }

    fn apply_instruction(&mut self, inst: &Instruction) {
        match inst.action {
            'N' => self.north_south -= inst.value,
            'S' => self.north_south += inst.value,
            'E' => self.east_west -= inst.value,
            'W' => self.east_west += inst.value,
            'R' => self.rotate_right(inst),
            'L' => self.rotate_left(inst),
            'F' => self.go_forward(inst.value),
            _ => unreachable!(),
        }
    }

    fn go_forward(&mut self, value: i64) {
        match self.direction {
            'N' => self.north_south -= value,
            'S' => self.north_south += value,
            'E' => self.east_west -= value,
            'W' => self.east_west += value,
            _ => unreachable!(),
        }
    }

    fn rotate_right(&mut self, inst: &Instruction) {
        let dirs = vec!['N', 'E', 'S', 'W'];
        let current_index = dirs.iter().position(|&r| r == self.direction).unwrap();
        let next_index = (current_index + (inst.value as usize / 90)) % dirs.len();
        self.direction = dirs[next_index];
    }

    fn rotate_left(&mut self, inst: &Instruction) {
        let dirs = vec!['N', 'W', 'S', 'E'];
        let current_index = dirs.iter().position(|&r| r == self.direction).unwrap();
        let next_index = (current_index + (inst.value as usize / 90)) % dirs.len();
        self.direction = dirs[next_index];
    }
}

struct WaypointFerry {
    north_south: i64, //north is negative, south is positive
    east_west: i64,   //east is negative, west is positive
    waypoint_north_south: i64,
    waypoint_east_west: i64,
}

impl BoatyMcBoatFace for WaypointFerry {
    fn manhattan_distance(&self) -> i64 {
        self.north_south.abs() + self.east_west.abs()
    }

    fn apply_instruction(&mut self, inst: &Instruction) {
        match inst.action {
            'N' => self.waypoint_north_south -= inst.value,
            'S' => self.waypoint_north_south += inst.value,
            'E' => self.waypoint_east_west -= inst.value,
            'W' => self.waypoint_east_west += inst.value,
            'R' => self.rotate_right(inst),
            'L' => self.rotate_left(inst),
            'F' => self.go_forward(inst.value),
            _ => unreachable!(),
        }
    }

    // Towards Waypoint
    fn go_forward(&mut self, value: i64) {
        self.north_south += value * self.waypoint_north_south;
        self.east_west += value * self.waypoint_east_west;
    }

    //Rotate Waypoint
    fn rotate_right(&mut self, inst: &Instruction) {
        let old_ew = self.waypoint_east_west;
        let old_ns = self.waypoint_north_south;

        match inst.value {
            90 => {
                self.waypoint_east_west = old_ns;
                self.waypoint_north_south = -1 * old_ew;
            }
            180 => {
                self.waypoint_east_west = -1 * old_ew;
                self.waypoint_north_south = -1 * old_ns;
            }
            270 => {
                self.rotate_left(&Instruction {
                    action: 'L',
                    value: 90,
                });
            }
            _ => unreachable!(),
        }
    }

    //Rotate Waypoint
    fn rotate_left(&mut self, inst: &Instruction) {
        let old_ew = self.waypoint_east_west;
        let old_ns = self.waypoint_north_south;

        match inst.value {
            90 => {
                self.waypoint_east_west = -1 * old_ns;
                self.waypoint_north_south = old_ew;
            }
            180 => {
                self.waypoint_east_west = -1 * old_ew;
                self.waypoint_north_south = -1 * old_ns;
            }
            270 => {
                self.rotate_right(&Instruction {
                    action: 'R',
                    value: 90,
                });
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("day12.txt");
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(parse_instruction(line));
    }

    let mut ferry = Ferry {
        direction: 'E',
        north_south: 0,
        east_west: 0,
    };
    for inst in instructions.iter() {
        ferry.apply_instruction(inst);
    }
    println!("Part 1: {}", ferry.manhattan_distance());

    let mut waypoint_ferry = WaypointFerry {
        north_south: 0,
        east_west: 0,
        waypoint_north_south: -1,
        waypoint_east_west: -10,
    };
    for inst in instructions.iter() {
        waypoint_ferry.apply_instruction(inst);
    }
    println!("Part 2: {}", waypoint_ferry.manhattan_distance());
}
