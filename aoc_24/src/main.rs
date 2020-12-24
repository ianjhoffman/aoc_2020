use std::collections::{HashMap, HashSet, VecDeque};
use util::file::GenericParseError;
use util::res::Result;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct HexCoord {
    x: i64, // lowest (one) bit represents half coordinates
    y: i64, // lowest (one) bit represents half coordinates
}

impl std::str::FromStr for HexCoord {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let e = Err(GenericParseError::ValueError(format!("Invalid line: {}", s).to_owned()));
        let mut coord = HexCoord{ x: 0, y: 0 };
        let mut chars = s.chars().collect::<VecDeque<char>>();
        while !chars.is_empty() {
            let offset = match chars.pop_front().unwrap() {
                'n' => {
                    match chars.pop_front() {
                        Some('e') => (1, 1),
                        Some('w') => (-1, 1),
                        _ => return e,
                    }
                },
                's' => {
                    match chars.pop_front() {
                        Some('e') => (1, -1),
                        Some('w') => (-1, -1),
                        _ => return e,
                    }
                },
                'e' => (2, 0),
                'w' => (-2, 0),
                _ => return e,
            };
            coord.x += offset.0;
            coord.y += offset.1;
        }

        Ok(coord)
    }
}

impl HexCoord {
    fn get_adjacent(&self) -> Box<dyn Iterator<Item = HexCoord>> {
        Box::new(vec![
            HexCoord{ x: self.x + 2, y: self.y }, // e
            HexCoord{ x: self.x - 2, y: self.y }, // w
            HexCoord{ x: self.x + 1, y: self.y + 1 }, // ne
            HexCoord{ x: self.x - 1, y: self.y + 1 }, // nw
            HexCoord{ x: self.x + 1, y: self.y - 1 }, // se
            HexCoord{ x: self.x - 1, y: self.y - 1 }, // sw
        ].into_iter())
    }
}

// Returns the input to part 2
fn part1(coords: &Vec<HexCoord>) -> HashSet<HexCoord> {
    let mut flipped: HashSet<HexCoord> = HashSet::new();
    for coord in coords {
        if flipped.contains(coord) {
            flipped.remove(coord);
        } else {
            flipped.insert(coord.clone());
        }
    }

    println!("[Part 1] # tiles left flipped at end: {}", flipped.len());
    flipped
}

fn part2(start_state: &HashSet<HexCoord>) {
    let final_state = (0..100).fold(start_state.clone(), |curr_state, _| {
        // Get counts of adjacent flipped tiles for all relevant coords
        let mut adjacent_counts: HashMap<HexCoord, usize> = HashMap::new();
        for coord in &curr_state {
            for adjacent in coord.get_adjacent() {
                *adjacent_counts.entry(adjacent).or_insert(0) += 1;
            }
        }

        // Update state based on adjacency flipping rules
        adjacent_counts.into_iter().filter(|(c, n)| {
            match (*n, curr_state.contains(c)) {
                (1, true) | (2, _) => true,
                _ => false,
            }
        }).map(|(c, _)| c).collect::<HashSet<HexCoord>>()
    });

    println!("[Part 2] # tiles left flipped after 100 days: {}", final_state.len());
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let coords = util::file::read_lines_to_type::<HexCoord>(file_path)?;

    let start_state = part1(&coords);
    part2(&start_state);
    Ok(())
}