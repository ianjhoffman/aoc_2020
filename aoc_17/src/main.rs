#[macro_use] extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use util::res::Result;

trait Coordinate {
    fn from_3d(input: (i64, i64, i64)) -> Self;
    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>>;
}

#[derive(PartialEq, Eq, std::hash::Hash)]
struct Coordinate3D {
    coords: (i64, i64, i64),
}

impl Coordinate for Coordinate3D {
    fn from_3d(input: (i64, i64, i64)) -> Self {
        Coordinate3D{ coords: input }
    }

    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>> {
        lazy_static! {
            static ref ADJACENCY_OFFSETS: HashSet<(i64, i64, i64)> = {
                let mut out: HashSet<(i64, i64, i64)> = HashSet::new();
                for x in -1..=1 {
                    for y in -1..=1 {
                        for z in -1..=1 {
                            out.insert((x, y, z));
                        }
                    }
                }
                out.remove(&(0, 0, 0));
                out
            };
        };

        let coords = self.clone().coords;
        Box::new(ADJACENCY_OFFSETS.iter().map(move |(ox, oy, oz)| {
            Coordinate3D{ coords: (coords.0 + ox, coords.1 + oy, coords.2 + oz) }
        }))
    }
}

#[derive(PartialEq, Eq, std::hash::Hash)]
struct Coordinate4D {
    coords: (i64, i64, i64, i64),
}

impl Coordinate for Coordinate4D {
    fn from_3d(input: (i64, i64, i64)) -> Self {
        Coordinate4D{ coords: (input.0, input.1, input.2, 0) }
    }

    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>> {
        lazy_static! {
            static ref ADJACENCY_OFFSETS: HashSet<(i64, i64, i64, i64)> = {
                let mut out: HashSet<(i64, i64, i64, i64)> = HashSet::new();
                for x in -1..=1 {
                    for y in -1..=1 {
                        for z in -1..=1 {
                            for w in -1..=1 {
                                out.insert((x, y, z, w));
                            }
                        }
                    }
                }
                out.remove(&(0, 0, 0, 0));
                out
            };
        };

        let coords = self.clone().coords;
        Box::new(ADJACENCY_OFFSETS.iter().map(move |(ox, oy, oz, ow)| {
            Coordinate4D{ coords: (coords.0 + ox, coords.1 + oy, coords.2 + oz, coords.3 + ow) }
        }))
    }
}

fn transition_n<T>(starting_state: &HashSet<(i64, i64, i64)>, iterations: usize) -> HashSet<T>
    where T: Coordinate + Eq + std::hash::Hash {
    let starting = starting_state.iter().cloned().map(|coords| {
        T::from_3d(coords)
    }).collect();

    (0..iterations).fold(starting, |acc, _| {
        // Set adjacency counts for all coordinates in 3D space next to active coordinates
        let mut adjacency_counts: HashMap<T, usize> = HashMap::new();
        for active_coords in &acc {
            for adjacent in active_coords.get_adjacent_coordinates() {
                *adjacency_counts.entry(adjacent).or_insert(0) += 1;
            }
        }

        adjacency_counts.into_iter().filter_map(|(coords, num_adjacent)| {
            match acc.contains(&coords) {
                true if (2..=3).contains(&num_adjacent) => Some(coords),
                false if num_adjacent == 3 => Some(coords),
                _ => None,
            }
        }).collect()
    })
}

fn part1(starting_state: &HashSet<(i64, i64, i64)>) {
    println!("[Part 1] Active cubes after 6 cycles: {}", transition_n::<Coordinate3D>(starting_state, 6).len());
}

fn part2(starting_state: &HashSet<(i64, i64, i64)>) {
    println!("[Part 2] Active cubes after 6 cycles: {}", transition_n::<Coordinate4D>(starting_state, 6).len());
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let starting_state = util::file::read_to_string(file_path)?.lines()
        .into_iter().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x, _)| (x as i64, y as i64, 0i64))
    }).collect::<HashSet<(i64, i64, i64)>>();

    part1(&starting_state);
    part2(&starting_state);
    Ok(())
}