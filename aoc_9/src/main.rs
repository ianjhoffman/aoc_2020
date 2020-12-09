use std::path::PathBuf;
use std::collections::HashSet;
use std::cmp::Ordering;
use util::res::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

struct Number {
    n: u64,
}

impl std::str::FromStr for Number{
    type Err = util::file::GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Number{n: s.parse::<u64>()?})
    }
}

fn part1(numbers: &Vec<u64>) -> Result<u64> {
    let mut seen: HashSet<u64> = HashSet::new();
    for (idx, n) in numbers.iter().enumerate() {
        seen.insert(*n);
        if idx < 25 { continue } // Skip preamble for sum checks

        // Do sum check - this whole function ends up being O(n^2)
        let none_found = seen.iter().map(|m| {
            if m >= n { return None }
            seen.get(&(n - m))
        }).all(|other_num| other_num.is_none());

        if none_found {
            println!("[Part 1] No 2 previous numbers found that sum to {}!", n);
            return Ok(*n);
        }
    }

    Err(From::from("[Part 1] All numbers could be summed to be 2 previous numbers!"))
}

fn part2(numbers: &Vec<u64>, unsummable: u64) {
    let (mut start_idx, mut end_idx, mut sum): (usize, usize, u64) = (0, 0, numbers[0]);
    while end_idx < numbers.len() - 1 || sum > unsummable {
        match sum.cmp(&unsummable) {
            Ordering::Less => {
                end_idx += 1;
                sum += numbers[end_idx];
            },
            Ordering::Greater => {
                sum -= numbers[start_idx];
                start_idx += 1;
                if start_idx > end_idx {
                    end_idx = start_idx;
                    sum = numbers[start_idx];
                }
            },
            Ordering::Equal => {
                let min_in_range = numbers[start_idx..=end_idx].iter().min().unwrap();
                let max_in_range = numbers[start_idx..=end_idx].iter().max().unwrap();
                println!(
                    "[Part 2] Found contiguous range (indices {}-{}) adding to {}. Min = {}, Max = {}, Weakness = {}",
                    start_idx, end_idx, unsummable, min_in_range, max_in_range, min_in_range + max_in_range,
                );
                return;
            }
        }
    }

    println!("[Part 2] Found no contiguous range adding to {}!", unsummable);
}

fn main() -> Result<()> {
    let opt = Cli::from_args();
    let numbers: Vec<u64> = util::file::read_lines_to_type::<Number>(opt.file)?
        .into_iter().map(|num| num.n).collect();

    let unsummable = part1(&numbers)?;
    part2(&numbers, unsummable);
    Ok(())
}