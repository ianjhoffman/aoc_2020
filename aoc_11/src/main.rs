use std::collections::HashMap;
use util::res::Result;
use util::file::GenericParseError;

// TileTransitionRule implements the logic for generating the next state of a seating tile
trait TileTransitionRule {
    // `get_next_state` takes the current grid state, a row index, and a columnn index
    // and returns the new state of the tile at that index and whether or not it changed
    fn get_next_state(&self, rows: &Vec<Row>, row_idx: usize, col_idx: usize) -> (Tile, bool);

    // `get_to_check` finds the list of tile (row, col) indices to check for each tile
    fn get_to_check(rows: &Vec<Row>, row_idx: usize, col_idx: usize) -> Vec<(usize, usize)>;

    // `get_to_check_map` returns a map from each tile (row, col) index to the files it
    // will check at each iteration of the simulation
    fn get_to_check_map(rows: &Vec<Row>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        (0..rows.len()).map(|row_idx| (0..rows[0].tiles.len()).map(move |col_idx| {
            ((row_idx, col_idx), Self::get_to_check(rows, row_idx, col_idx))
        })).flatten().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied
}

#[derive(Debug, Clone)]
struct Row {
    tiles: Vec<Tile>
}

impl std::str::FromStr for Row {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Row{
            tiles: s.chars().map(|c| match c {
                '.' => Ok(Tile::Floor),
                'L' => Ok(Tile::Empty),
                '#' => Ok(Tile::Occupied),
                _ => Err(GenericParseError::ValueError(format!("Invalid character: {}", c).to_owned())),
            }).collect::<std::result::Result<Vec<Tile>, Self::Err>>()?,
        })
    }
}

fn get_occupied(rows: &Vec<Row>, to_check: &Vec<(usize, usize)>) -> usize {
    to_check.iter().fold(0, |acc, &(row_idx, col_idx)| {
        if rows[row_idx].tiles[col_idx] == Tile::Occupied { acc + 1 } else { acc }
    })
}

// Definitions for the adjacency transition rule

struct AdjacencyRule {
    to_check: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl TileTransitionRule for AdjacencyRule {
    fn get_next_state(&self, rows: &Vec<Row>, row_idx: usize, col_idx: usize) -> (Tile, bool) {
        let to_check = self.to_check.get(&(row_idx, col_idx)).unwrap();
        let occupied_adjacent = get_occupied(rows, to_check);
        match &rows[row_idx].tiles[col_idx] {
            Tile::Empty if (occupied_adjacent == 0) => (Tile::Occupied, true),
            Tile::Occupied if (occupied_adjacent >= 4) => (Tile::Empty, true),
            t => (t.clone(), false),
        }
    }

    fn get_to_check(rows: &Vec<Row>, row_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
        let row_range = 0..(rows.len() as i64);
        let col_range = 0..(rows[0].tiles.len() as i64);

        vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].into_iter()
            .filter_map(|(row_offset, col_offset)| {
                let check_row = row_offset + row_idx as i64;
                let check_col = col_offset + col_idx as i64;
                match row_range.contains(&check_row) && col_range.contains(&check_col) {
                    true => {
                        Some((check_row as usize, check_col as usize))
                    }
                    false => None
                }
            }).collect()
    }
}

impl AdjacencyRule {
    fn new(rows: &Vec<Row>) -> Self {
        AdjacencyRule{ to_check: Self::get_to_check_map(rows) }
    }
}

// Definitions for the line-of-sight transition rule

struct LineOfSightRule {
    to_check: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl TileTransitionRule for LineOfSightRule {
    fn get_next_state(&self, rows: &Vec<Row>, row_idx: usize, col_idx: usize) -> (Tile, bool) {
        let to_check = self.to_check.get(&(row_idx, col_idx)).unwrap();
        let occupied_adjacent = get_occupied(rows, to_check);
        match &rows[row_idx].tiles[col_idx] {
            Tile::Empty if (occupied_adjacent == 0) => (Tile::Occupied, true),
            Tile::Occupied if (occupied_adjacent >= 5) => (Tile::Empty, true),
            t => (t.clone(), false),
        }
    }

    fn get_to_check(rows: &Vec<Row>, row_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
        let row_range = 0..(rows.len() as i64);
        let col_range = 0..(rows[0].tiles.len() as i64);

        vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].into_iter()
            .filter_map(|(row_offset, col_offset)| {
                for direction_multiplier in 1.. {
                    let check_row = (direction_multiplier * row_offset) + (row_idx as i64);
                    let check_col = (direction_multiplier * col_offset) + (col_idx as i64);
                    if !(row_range.contains(&check_row) && col_range.contains(&check_col)) {
                        return None
                    }

                    if rows[check_row as usize].tiles[check_col as usize] != Tile::Floor {
                        return Some((check_row as usize, check_col as usize))
                    }
                }
                None
            }).collect()
    }
}

impl LineOfSightRule {
    fn new(rows: &Vec<Row>) -> Self {
        LineOfSightRule{ to_check: Self::get_to_check_map(rows) }
    }
}

fn simulate_until_stable(rows: &Vec<Row>, rule: impl TileTransitionRule) -> Vec<Row> {
    let mut prev = rows.clone();
    let mut curr = vec![Row{ tiles: vec![Tile::Empty; rows[0].tiles.len()]}; rows.len()];
    loop {
        let mut num_changed = 0;
        curr.iter_mut().enumerate().for_each(|(row_idx, row)| {
            row.tiles.iter_mut().enumerate().for_each(|(col_idx, tile)| {
                let (new, changed) = rule.get_next_state(&prev, row_idx, col_idx);
                *tile = new;
                if changed { num_changed += 1; }
            })
        });

        if num_changed == 0 { break; }
        prev = curr.clone();
    }
    curr
}

fn get_occupied_count_in_stable_arrangement(rows: &Vec<Row>, rule: impl TileTransitionRule) -> usize {
    simulate_until_stable(rows, rule).iter().fold(0, |acc, r| {
        acc + r.tiles.iter().filter(|&t| t == &Tile::Occupied).count()
    })
}

fn part1(rows: &Vec<Row>) {
    let num_occupied = get_occupied_count_in_stable_arrangement(rows, AdjacencyRule::new(rows));
    println!("[Part 1] Seats occupied after stable seating arrangment is found: {}", num_occupied);
}

fn part2(rows: &Vec<Row>) {
    let num_occupied = get_occupied_count_in_stable_arrangement(rows, LineOfSightRule::new(rows));
    println!("[Part 2] Seats occupied after stable seating arrangment is found: {}", num_occupied);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let rows = util::file::read_lines_to_type::<Row>(file_path)?;

    part1(&rows);
    part2(&rows);
    Ok(())
}