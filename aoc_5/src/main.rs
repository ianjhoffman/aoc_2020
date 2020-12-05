use std::path::PathBuf;
use std::collections::HashSet;
use util::res::Result;
use util::file::GenericParseError;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

struct BoardingSeat {
    row: u8,
    col: u8,
}

impl std::str::FromStr for BoardingSeat {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(GenericParseError::ValueError(format!("Invalid length: {}", s).to_owned()));
        }

        let row_chars_and_bit_idx = s.chars().take(7).zip((0..=6).rev());
        let col_chars_and_bit_idx = s.chars().skip(7).zip((0..=2).rev());
        Ok(BoardingSeat{
            row: row_chars_and_bit_idx.filter(|(c, _)| *c == 'B').fold(0, |acc, (_, idx)| acc | 1 << idx),
            col: col_chars_and_bit_idx.filter(|(c, _)| *c == 'R').fold(0, |acc, (_, idx)| acc | 1 << idx),
        })
    }
}

impl BoardingSeat {
    fn get_id(&self) -> u16 {
        (self.row as u16) * 8 + (self.col as u16)
    }
}

fn part1(boarding_seats: &Vec<BoardingSeat>) {
    let max_seat_id = boarding_seats.iter().map(|s| s.get_id()).max().unwrap_or(0);
    println!("[Part 1] Max seat ID: {}", max_seat_id);
}

fn part2(boarding_seats: &Vec<BoardingSeat>) {
    let seat_id_set: HashSet<u16> = boarding_seats.iter().map(|s| s.get_id()).collect::<HashSet<u16>>();
    let missing_seat_id = (|| {
        for row in 1..=126 {
            for col in 0..=7 {
                let seat_id = BoardingSeat{row, col}.get_id();
                if !seat_id_set.contains(&seat_id) {
                    if seat_id_set.contains(&(seat_id - 1)) && seat_id_set.contains(&(seat_id + 1)) {
                        return Some(seat_id);
                    }
                }
            }
        }

        None
    })();

    if let Some(seat_id) = missing_seat_id {
        println!("[Part 2] Your seat ID is {}", seat_id);
    } else {
        println!("[Part 2] No valid missing seat ID found!");
    }
}

fn main() -> Result<()> {
    let opt = Cli::from_args();
    let boarding_seats: Vec<BoardingSeat> = util::file::read_lines_to_type::<BoardingSeat>(opt.file)?;

    part1(&boarding_seats);
    part2(&boarding_seats);

    Ok(())
}