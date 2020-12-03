use std::path::PathBuf;
use util::res::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

struct Row {
    tree_pattern: Vec<bool>,
}

impl std::str::FromStr for Row {
    type Err = util::file::GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Row{ tree_pattern: s.chars().map(|c| c == '#').collect() })
    }
}

fn count_trees_at_angle(rows: &Vec<Row>, right: usize, down: usize) -> usize {
    let modulus = rows[0].tree_pattern.len();
    rows.iter().step_by(down).enumerate().filter(|(step_count, row)| {
        row.tree_pattern[(step_count * right) % modulus]
    }).count()
}

fn part1(rows: &Vec<Row>) {
    println!("[Part 1]: # trees at angle (right 3, down 1): {}", count_trees_at_angle(rows, 3, 1));
}

fn part2(rows: &Vec<Row>) {
    let slopes_to_check: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let product: usize = slopes_to_check.iter().fold(1, |acc, slope| {
        acc * count_trees_at_angle(rows, slope.0, slope.1)
    });

    println!("[Part 2]: Product of all slopes' tree counts: {}", product);
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let rows: Vec<Row> = util::file::read_lines_to_type::<Row>(opt.file)?;

    part1(&rows);
    part2(&rows);
    Ok(())
}