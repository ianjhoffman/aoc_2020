use std::collections::HashMap;
use util::res::Result;

fn find_two_entries_summing_to_num(expenses: &Vec<u32>, num: u32, ignore: Option<usize>) -> Option<(u32, u32)> {
    let seen: HashMap<u32, usize> = expenses.iter().enumerate().map(|(idx, &e)| (e, idx)).collect();
    for (idx, &expense) in expenses.iter().enumerate() {
        if expense > num { continue; }
        if Some(idx) == ignore { continue; }

        match seen.get(&(num - expense)) {
            Some(seen_idx) if *seen_idx != idx => return Some((num - expense, expense)),
            _ => (),
        }
    }
    
    None
}

fn part1(expenses: &Vec<u32>) {
    match find_two_entries_summing_to_num(expenses, 2020, None) {
        Some((a, b)) => println!("Found numbers {}, {} - product = {}", a, b, a * b),
        None => println!("Did not find any pair of numbers summing to 2020")
    }
}

fn part2(expenses: &Vec<u32>) {
    for (idx, expense) in expenses.iter().enumerate() {
        let a = expense;
        let remaining = 2020 - expense;
        if let Some((b, c)) = find_two_entries_summing_to_num(expenses, remaining, Some(idx)) {
            println!("Found numbers {}, {}, {} - product = {}", a, b, c, a*b*c);
            return;
        }
    }

    println!("Did not find any triple of numbers summing to 2020")
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let expenses = util::file::read_lines_to_integers::<u32>(file_path)?;

    part1(&expenses);
    part2(&expenses);
    Ok(())
}