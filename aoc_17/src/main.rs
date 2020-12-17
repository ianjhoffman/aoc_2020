#[macro_use] extern crate lazy_static;
use std::collections::{HashMap, HashSet};
use util::res::Result;

trait Coordinate {
    fn from_3d(input: (i64, i64, i64)) -> Self;
    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>>;
}

impl Coordinate for [i64; 3] {
    fn from_3d(input: (i64, i64, i64)) -> Self { [input.0, input.1, input.2] }
    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>> {
        lazy_static! {
            static ref ADJACENCY_OFFSETS: HashSet<[i64; 3]> = {
                let mut out: HashSet<[i64; 3]> = vec![[0, 0, 0]].into_iter().collect();
                for idx in 0..3 {
                    out = out.into_iter().flat_map(|incomplete| (-1..=1).map(move |offset| {
                        let mut cloned = incomplete.clone();
                        cloned[idx] = offset;
                        cloned
                    })).collect();
                }
                out.remove(&[0, 0, 0]);
                out
            };
        };

        let c = self.clone();
        Box::new(ADJACENCY_OFFSETS.iter().map(move |o| [c[0] + o[0], c[1] + o[1], c[2] + o[2]]))
    }
}

impl Coordinate for [i64; 4] {
    fn from_3d(input: (i64, i64, i64)) -> Self { [input.0, input.1, input.2, 0] }
    fn get_adjacent_coordinates(&self) -> Box<dyn Iterator<Item = Self>> {
        lazy_static! {
            static ref ADJACENCY_OFFSETS: HashSet<[i64; 4]> = {
                let mut out: HashSet<[i64; 4]> = vec![[0, 0, 0, 0]].into_iter().collect();
                for idx in 0..4 {
                    out = out.into_iter().flat_map(|incomplete| (-1..=1).map(move |offset| {
                        let mut cloned = incomplete.clone();
                        cloned[idx] = offset;
                        cloned
                    })).collect();
                }
                out.remove(&[0, 0, 0, 0]);
                out
            };
        };

        let c = self.clone();
        Box::new(ADJACENCY_OFFSETS.iter().map(move |o| [c[0] + o[0], c[1] + o[1], c[2] + o[2], c[3] + o[3]]))
    }
}

fn transition_n<T>(starting_state: &HashSet<(i64, i64, i64)>, iterations: usize) -> HashSet<T>
    where T: Coordinate + Eq + std::hash::Hash {
    (0..iterations).fold(starting_state.iter().cloned().map(|coords| T::from_3d(coords)).collect(), 
    |acc, _| {
        // Set adjacency counts for all coordinates in 3D space next to active coordinates
        let mut adjacency_counts: HashMap<T, usize> = HashMap::new();
        for active_coords in &acc {
            for adjacent in active_coords.get_adjacent_coordinates() {
                *adjacency_counts.entry(adjacent).or_insert(0) += 1;
            }
        }

        adjacency_counts.into_iter().filter_map(|(coords, num_adjacent)| {
            match (acc.contains(&coords), num_adjacent) {
                (true, 2) | (true, 3) | (false, 3) => Some(coords),
                _ => None,
            }
        }).collect()
    })
}

fn part1(starting_state: &HashSet<(i64, i64, i64)>) {
    println!("[Part 1] Active cubes after 6 cycles: {}", transition_n::<[i64; 3]>(starting_state, 6).len());
}

fn part2(starting_state: &HashSet<(i64, i64, i64)>) {
    println!("[Part 2] Active cubes after 6 cycles: {}", transition_n::<[i64; 4]>(starting_state, 6).len());
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