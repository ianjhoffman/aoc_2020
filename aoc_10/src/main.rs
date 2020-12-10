use util::res::Result;

fn part1(joltages: &Vec<u64>) {
    let (mut diffs_1, mut diffs_3, mut prev): (u64, u64, u64) = (0, 0, 0);
    for joltage in joltages {
        match *joltage - prev {
            1 => diffs_1 += 1,
            3 => diffs_3 += 1,
            _ => (),
        }
        prev = *joltage;
    }

    diffs_3 += 1; // Final 3-joltage difference
    println!("[Part 1] 1-Jolt differences: {}, 3-Jolt differences: {}, Product: {}", diffs_1, diffs_3, diffs_1 * diffs_3);
}

fn part2(joltages: &Vec<u64>) {
    let mut arrangements_from_each: Vec<u64> = vec![0; joltages.len()];

    // The highest-rated adapter can only do one arrangement - plugging into your device
    arrangements_from_each[joltages.len() - 1] = 1;

    for idx in (0..joltages.len() - 1).rev() {
        // Get indices of accessible subsequent adapters with joltage rating difference <= 3
        let accessible_indices: Vec<usize> = joltages.iter().enumerate().skip(idx + 1)
            .take_while(|(_, &j)| j - joltages[idx] <= 3).map(|(later, _)| later).collect();
        
        // Add up all arrangements from accessible subsequent adapters to the end
        arrangements_from_each[idx] = accessible_indices.iter().fold(0, |acc, &later| {
            acc + if later == joltages.len() { 1 } else { arrangements_from_each[later] }
        })
    }

    let arrangements_from_zero = joltages.iter().take_while(|&&j| j <= 3) 
        .enumerate().fold(0, |acc, (idx, _)| acc + arrangements_from_each[idx]);

    println!("[Part 2] Possible arrangements: {}", arrangements_from_zero);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let mut joltages = util::file::read_lines_to_integers::<u64>(file_path)?;
    joltages.sort();

    part1(&joltages);
    part2(&joltages);
    Ok(())
}