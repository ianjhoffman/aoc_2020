#[macro_use] extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use util::res::Result;

trait Coordinate {
    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>>;
}

#[derive(PartialEq, Eq, std::hash::Hash)]
struct Coordinate3D {
    coords: (i64, i64, i64),
}

impl Coordinate for Coordinate3D {
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

fn transition<T>(state: &HashSet<T>) -> HashSet<T> where T: Coordinate + Eq + std::hash::Hash {
    // Set adjacency counts for all coordinates in 3D space next to active coordinates
    let mut adjacency_counts: HashMap<T, usize> = HashMap::new();
    for active_coords in state {
        for adjacent in active_coords.get_adjacent_coordinates() {
            *adjacency_counts.entry(adjacent).or_insert(0) += 1;
        }
    }

    adjacency_counts.into_iter().filter_map(|(coords, num_adjacent)| {
        match state.contains(&coords) {
            true if (2..=3).contains(&num_adjacent) => Some(coords),
            false if num_adjacent == 3 => Some(coords),
            _ => None,
        }
    }).collect()
}

fn part1(starting_state: &HashSet<(i64, i64, i64)>) {
    let starting_3d = starting_state.iter().cloned().map(|coords| {
        Coordinate3D{ coords }
    }).collect();

    let after_six = (0..6).fold(starting_3d, |acc, _| {
        transition::<Coordinate3D>(&acc)
    });

    println!("[Part 1] Active cubes after 6 cycles: {}", after_six.len());
}

fn part2(starting_state: &HashSet<(i64, i64, i64)>) {
    let starting_4d = starting_state.iter().cloned().map(|coords| {
        Coordinate4D{ coords: (coords.0, coords.1, coords.2, 0) }
    }).collect();

    let after_six = (0..6).fold(starting_4d, |acc, _| {
        transition::<Coordinate4D>(&acc)
    });

    println!("[Part 2] Active cubes after 6 cycles: {}", after_six.len());
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