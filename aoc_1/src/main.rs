use std::path::PathBuf;
use std::collections::HashMap;
use util::res::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

struct Expense {
    amount: u32
}

impl std::str::FromStr for Expense {
    type Err = util::file::GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Expense{ amount: s.parse::<u32>()? })
    }
}

fn find_two_entries_summing_to_num(expenses: &Vec<Expense>, num: u32, ignore: Option<usize>) -> Option<(u32, u32)> {
    let seen: HashMap<u32, usize> = expenses.iter().enumerate().map(|(idx, e)| (e.amount, idx)).collect();
    for (idx, expense) in expenses.iter().enumerate() {
        if expense.amount > num { continue; }
        if Some(idx) == ignore { continue; }

        match seen.get(&(num - expense.amount)) {
            Some(seen_idx) if *seen_idx != idx => return Some((num - expense.amount, expense.amount)),
            _ => (),
        }
    }
    
    None
}

fn part1(expenses: &Vec<Expense>) {
    match find_two_entries_summing_to_num(expenses, 2020, None) {
        Some((a, b)) => println!("Found numbers {}, {} - product = {}", a, b, a * b),
        None => println!("Did not find any pair of numbers summing to 2020")
    }
}

fn part2(expenses: &Vec<Expense>) {
    for (idx, expense) in expenses.iter().enumerate() {
        let a = expense.amount;
        let remaining = 2020 - expense.amount;
        if let Some((b, c)) = find_two_entries_summing_to_num(expenses, remaining, Some(idx)) {
            println!("Found numbers {}, {}, {} - product = {}", a, b, c, a*b*c);
            return;
        }
    }

    println!("Did not find any triple of numbers summing to 2020")
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let expenses = util::file::read_lines_to_type::<Expense>(opt.file)?;
    part1(&expenses);
    part2(&expenses);

    Ok(())
}