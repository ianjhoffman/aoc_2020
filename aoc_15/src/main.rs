use std::collections::HashMap;
use util::res::Result;

fn get_nth_number_spoken(starting_numbers: &Vec<u64>, n: usize) -> u64 {
    let mut prev = 0u64;
    let mut last_spoken: HashMap<u64, (Option<u64>, u64)> = HashMap::new();
    for i in 0..n {
        prev = if i < starting_numbers.len() {
            starting_numbers[i]
        } else {
            match last_spoken.get(&prev).unwrap() {
                (Some(last_last), last) => last - last_last,
                (None, _) => 0,
            }
        };

        // Update last_spoken now that `prev` has been spoken
        last_spoken.insert(prev, match last_spoken.get(&prev) {
            None => (None, i as u64),
            Some((_, last)) => (Some(*last), i as u64),
        });
    }

    prev
}

fn part1(starting_numbers: &Vec<u64>) {
    println!("[Part 1] 2020th number spoken: {}", get_nth_number_spoken(starting_numbers, 2020));
}

fn part2(starting_numbers: &Vec<u64>) {
    println!("[Part 2] 30,000,000th number spoken: {}", get_nth_number_spoken(starting_numbers, 30_000_000));
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let starting_numbers= util::file::read_lines_to_integers::<u64>(file_path)?;

    part1(&starting_numbers);
    part2(&starting_numbers);
    Ok(())
}