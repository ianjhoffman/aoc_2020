use std::collections::{HashMap, HashSet, VecDeque};
use util::res::Result;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Side {
    Right,
    Left,
    Top,
    Bottom,
}

#[derive(Clone)]
struct Tile {
    id: u64,
    pixels: [[bool; 10]; 10],
    edges: HashMap<Side, [bool; 10]>,
}

impl Tile {
    fn rotate_left(&self) -> Tile {
        let mut out = Tile{ id: self.id, pixels: [[false; 10]; 10], edges: HashMap::new() };

        // Rotate edge mappings
        out.edges.insert(Side::Top, self.edges.get(&Side::Right).unwrap().clone());
        out.edges.insert(Side::Left, self.edges.get(&Side::Top).unwrap().clone());
        out.edges.insert(Side::Bottom, self.edges.get(&Side::Left).unwrap().clone());
        out.edges.insert(Side::Right, self.edges.get(&Side::Bottom).unwrap().clone());

        // Rotate pixels
        for row_idx in 0usize..10 {
            for col_idx in 0usize..10 {
                out.pixels[9 - col_idx][row_idx] = self.pixels[row_idx][col_idx];
            }
        }

        out
    }

    fn flip_horizontal(&self) -> Tile {
        let mut out = Tile{ id: self.id, pixels: [[false; 10]; 10], edges: HashMap::new() };

        // Flip edge mappings
        out.edges.insert(Side::Top, flip_edge(self.edges.get(&Side::Top).unwrap()));
        out.edges.insert(Side::Left, flip_edge(self.edges.get(&Side::Right).unwrap()));
        out.edges.insert(Side::Bottom, flip_edge(self.edges.get(&Side::Bottom).unwrap()));
        out.edges.insert(Side::Right, flip_edge(self.edges.get(&Side::Left).unwrap()));

        // Flip pixels
        for row_idx in 0usize..10 {
            out.pixels[row_idx] = flip_edge(&self.pixels[row_idx]);
        }

        out
    }
}

fn get_side(pixels: &[[bool; 10]; 10], side: Side) -> [bool; 10] {
    let mut out = [false; 10];
    let side_pixels = match side {
        Side::Left => (&pixels[..]).iter().map(|row| row[0]).collect::<Vec<bool>>(),
        Side::Right => (&pixels[..]).iter().rev().map(|row| row[9]).collect::<Vec<bool>>(),
        Side::Top => pixels[0].iter().rev().cloned().collect::<Vec<bool>>(),
        Side::Bottom => pixels[9].iter().cloned().collect::<Vec<bool>>(),
    };

    out.clone_from_slice(side_pixels.as_slice());
    out
}

fn to_tiles(contents: &String) -> Result<HashMap<u64, Tile>> {
    contents.lines().collect::<Vec<&str>>().chunks(12).map(|tile_lines| {
        let mut pixels: [[bool; 10]; 10] = [[false; 10]; 10];
        for (row, tile_line) in (&tile_lines[1..11]).iter().enumerate() {
            tile_line.chars().take(10).enumerate().for_each(|(col, c)| pixels[row][col] = c == '#');
        }

        let id = tile_lines[0].strip_prefix("Tile ").and_then(|rem| rem.strip_suffix(":"))
            .unwrap_or("INVALID TILE HEADER").parse::<u64>()?;

        Ok((id, Tile{
            id,
            pixels,
            edges: vec![Side::Top, Side::Bottom, Side::Left, Side::Right].into_iter().map(|s| {
                (s.clone(), get_side(&pixels, s))
            }).collect::<HashMap<Side, [bool; 10]>>(),
        }))
    }).collect::<Result<HashMap<u64, Tile>>>()
}

fn flip_edge(edge: &[bool; 10]) -> [bool; 10] {
    let mut edge_flipped = [false; 10];
    (0..10).for_each(|i| edge_flipped[i] = edge[9 - i]);
    edge_flipped
}

fn reorient_to_match(tile: &Tile, adjacent_to: Side, pixels: &[bool; 10], flipped: bool) -> Tile {
    let mut reorienting = tile.clone();
    if flipped { reorienting = reorienting.flip_horizontal(); }

    let matching_side = reorienting.edges.iter().find(|(_, p)| *p == pixels).unwrap().0;
    let desired_side = match adjacent_to {
        Side::Top => Side::Bottom,
        Side::Bottom => Side::Top,
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    };

    let rotations_required = vec![Side::Right, Side::Top, Side::Left, Side::Bottom].into_iter()
        .cycle().skip_while(|e| e != matching_side).enumerate()
        .skip_while(|(_, e)| *e != desired_side)
        .nth(0).unwrap().0;

    (0..rotations_required).fold(reorienting, |t, _| t.rotate_left())
}

struct PuzzleSolution {
    corner_id_product: u64,
    raw_image: HashMap<(usize, usize), bool>,
}

fn assembled_to_raw_image(
    assembled: &HashMap<(i64, i64), Tile>,
    lower_left: (i64, i64),
    upper_right: (i64, i64)
) -> HashMap<(usize, usize), bool> {
    let mut out: HashMap<(usize, usize), bool> = HashMap::new();
    for (i, tile_y) in ((lower_left.1)..=(upper_right.1)).rev().enumerate() {
        for (j, tile_x) in ((lower_left.0)..=(upper_right.0)).enumerate() {
            let tile = assembled.get(&(tile_x, tile_y)).unwrap();
            for pixel_row in 0usize..8 {
                for pixel_col in 0usize..8 {
                    let out_coords = (8 * i + pixel_row, 8 * j + pixel_col);
                    //println!("Inserting out coords... {:?}", out_coords);
                    out.insert(out_coords, tile.pixels[pixel_row + 1][pixel_col + 1]);
                }
            }
        }
    }

    out
}

fn solve_puzzle(tiles: &HashMap<u64, Tile>) -> PuzzleSolution {
    // Square side length constraint
    let side_length = (tiles.len() as f64).sqrt().round() as i64;

    // Group together tiles with common edges
    let mut edges_to_tile_ids: HashMap<[bool; 10], Vec<u64>> = HashMap::new();
    for (_, tile) in tiles {
        for (_, edge_pixels) in &tile.edges {
            edges_to_tile_ids.entry(edge_pixels.clone()).or_insert(vec![]).push(tile.id);
        }
    }

    let origin_tile = tiles.values().nth(0).unwrap();

    // Assemble the image
    let mut used: HashSet<u64> = vec![origin_tile.id].into_iter().collect();
    let mut assembled: HashMap<(i64, i64), Tile> = vec![((0, 0), origin_tile.clone())].into_iter().collect();
    let mut examine: VecDeque<(i64, i64)> = vec![(0, 0)].into_iter().collect();
    let mut lower_left: (i64, i64) = (0, 0);
    let mut upper_right: (i64, i64) = (0, 0);
    while examine.len() > 0 {
        let ex_coords = examine.pop_front().unwrap();
        let ex_tile = assembled.get(&ex_coords).unwrap().clone();

        for (neighbor_offset, neighbor_side) in vec![
            ((0, 1), Side::Top),
            ((0, -1), Side::Bottom),
            ((1, 0), Side::Right),
            ((-1, 0), Side::Left)
        ] {
            let neighbor_coords = (ex_coords.0 + neighbor_offset.0, ex_coords.1 + neighbor_offset.1);
            if assembled.contains_key(&neighbor_coords) { continue } // That spot is already filled

            // Make sure this doesn't make the puzzle too big
            let new_lower_left = (lower_left.0.min(neighbor_coords.0), lower_left.1.min(neighbor_coords.1));
            let new_upper_right = (upper_right.0.max(neighbor_coords.0), upper_right.1.max(neighbor_coords.1));
            if (1 + new_upper_right.0 - new_lower_left.0) > side_length { continue }
            if (1 + new_upper_right.1 - new_lower_left.1) > side_length { continue }

            // Find tile ID sharing common edge with the given neighbor side edge of this assembled tile
            let edge_pixels = ex_tile.edges.get(&neighbor_side).unwrap();
            let edge_pixels_rev = flip_edge(edge_pixels);

            // The corresponding side on an adjacent tile should have the reverse of this edge, since
            // we always spell out our edges in counterclockwise order on each tile
            for (pixels, flipped) in vec![(edge_pixels, true), (&edge_pixels_rev, false)] {
                let maybe_matching_tile_id = edges_to_tile_ids.get(pixels).map(|ids| {
                    ids.iter().find(|&tid| {
                        *tid != ex_tile.id && !used.contains(tid)
                    })
                }).flatten();
                
                match maybe_matching_tile_id {
                    Some(other_id) if !used.contains(other_id) => {
                        let reoriented = reorient_to_match(tiles.get(other_id).unwrap(), neighbor_side.clone(), &edge_pixels_rev, flipped);
                        used.insert(reoriented.id);
                        assembled.insert(neighbor_coords, reoriented);
                        examine.push_back(neighbor_coords);
                        lower_left = new_lower_left;
                        upper_right = new_upper_right;
                        break
                    },
                    _ => (),
                }
            }
        }
    }

    let (min_x, max_x, min_y, max_y) = (lower_left.0, upper_right.0, lower_left.1, upper_right.1);
    let product = vec![(min_x, min_y), (min_x, max_y), (max_x, min_y), (max_x, max_y)]
        .iter().fold(1, |acc, coord| {
            acc * assembled.get(coord).unwrap().id
        }
    );

    PuzzleSolution {
        corner_id_product: product,
        raw_image: assembled_to_raw_image(&assembled, lower_left, upper_right),
    }
}

fn part1(puzzle_solution: &PuzzleSolution) {
    println!("[Part 1] Product of corner tile IDs: {}", puzzle_solution.corner_id_product);
}

fn part2(puzzle_solution: &PuzzleSolution) {
    // TODO
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let tiles: HashMap<u64, Tile> = to_tiles(&contents)?;
    let puzzle_solution = solve_puzzle(&tiles);

    part1(&puzzle_solution);
    part2(&puzzle_solution);
    Ok(())
}