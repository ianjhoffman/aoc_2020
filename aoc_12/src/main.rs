use util::res::Result;
use util::file::GenericParseError;

#[derive(PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    action: Action,
    value: i64,
}

impl std::str::FromStr for Instruction {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let action = match s.chars().nth(0) {
            Some('N') => Action::North,
            Some('S') => Action::South,
            Some('E') => Action::East,
            Some('W') => Action::West,
            Some('L') => Action::Left,
            Some('R') => Action::Right,
            Some('F') => Action::Forward,
            _ => return Err(GenericParseError::ValueError(format!("Invalid instruction: {}", s).to_owned())),
        };

        let value = s.chars().skip(1).collect::<String>().parse::<i64>()?;
        Ok(Instruction{action, value})
    }
}

fn rotate_waypoint(waypoint: (i64, i64), degrees: i64, left: bool) -> (i64, i64) {
    let mut rotations = vec![
        (waypoint.0, waypoint.1), // 0 degrees
        (-waypoint.1, waypoint.0), // 90 degrees left
        (-waypoint.0, -waypoint.1), // 180 degrees left
        (waypoint.1, -waypoint.0), // 270 degrees left
    ];
    if !left { rotations.reverse(); }
    rotations.into_iter().cycle().skip_while(|coords| *coords != waypoint)
        .skip(degrees as usize / 90).nth(0).unwrap()
}

struct Ship {
    x: i64,
    y: i64,
    waypoint: (i64, i64),
    cardinal_directions_move_waypoint: bool,
}

impl Ship {
    fn new(waypoint: (i64, i64), cardinal_directions_move_waypoint: bool) -> Self {
        Ship{ x: 0, y: 0, waypoint, cardinal_directions_move_waypoint }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => if self.cardinal_directions_move_waypoint {
                self.waypoint.1 += instruction.value
            } else {
                self.y += instruction.value
            },
            Action::South => if self.cardinal_directions_move_waypoint {
                self.waypoint.1 -= instruction.value
            } else {
                self.y -= instruction.value
            },
            Action::East => if self.cardinal_directions_move_waypoint {
                self.waypoint.0 += instruction.value
            } else {
                self.x += instruction.value
            },
            Action::West => if self.cardinal_directions_move_waypoint {
                self.waypoint.0 -= instruction.value
            } else {
                self.x -= instruction.value
            },
            Action::Left | Action::Right => {
                self.waypoint = rotate_waypoint(
                    self.waypoint,
                    instruction.value,
                    instruction.action == Action::Left
                );
            },
            Action::Forward => {
                self.x += instruction.value * self.waypoint.0;
                self.y += instruction.value * self.waypoint.1;
            },
        }
    }
}

fn part1(instructions: &Vec<Instruction>) {
    let mut ship = Ship::new((1, 0), false); // Start east
    instructions.iter().for_each(|instr| ship.apply_instruction(instr));
    println!("[Part 1] Ship x = {}, Ship y = {}, Manhattan distance = {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
}

fn part2(instructions: &Vec<Instruction>) {
    let mut ship = Ship::new((10, 1), true);
    instructions.iter().for_each(|instr| ship.apply_instruction(instr));
    println!("[Part 2] Ship x = {}, Ship y = {}, Manhattan distance = {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let instructions = util::file::read_lines_to_type::<Instruction>(file_path)?;

    part1(&instructions);
    part2(&instructions);
    Ok(())
}