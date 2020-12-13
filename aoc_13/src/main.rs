use util::res::Result;

fn parse_input(contents: &String) -> Result<(u64, Vec<(usize, u64)>)> {
    let lines = contents.lines().collect::<Vec<&str>>();
    if lines.len() != 2 {
        return Err(format!("Invalid line count: {}", lines.len()).into())
    }

    let earliest = lines[0].parse::<u64>()?;
    let mut bus_ids = vec![];
    for (idx, relevant_id_str) in lines[1].split(',').enumerate().filter(|(_, id_str)| *id_str != "x") {
        bus_ids.push((idx, relevant_id_str.parse::<u64>()?));
    }
    
    Ok((earliest, bus_ids))
}

fn part1(earliest: u64, bus_ids: &Vec<(usize, u64)>) {
    let (closest, wait_mins): (u64, u64) = bus_ids.iter().map(|&(_, id)| (id, (id - (earliest % id)) % id))
        .min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    println!("[Part 1] Closest bus ID: {}, wait mins: {}, product: {}", closest, wait_mins, closest * wait_mins);
}

// Returns gcd, x, y such that ax + by = gcd
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
	if a == 0 { return (b, 0, 1); }
	let (gcd, x, y) = extended_gcd(b % a, a);
    (gcd, (y - (b/a) * x), x)
}

fn lcm(a: i128, b: i128) -> i128 {
    a * b / extended_gcd(a, b).0
}

// Solves for <a> in a linear congruence of the form <factor> * <a> ≡ <remainder> mod <modulus>
fn solve_linear_congruence(factor: i128, remainder: i128, modulus: i128) -> i128 {
    let (_, multiplicative_inverse, _) = extended_gcd(factor, modulus);
    (remainder * multiplicative_inverse).rem_euclid(modulus)
}

struct BusRelationInfo {
    first_bus_cycle_count: i128,
    subsequent_cycle_count: i128,
}

fn part2(bus_ids: &Vec<(usize, u64)>) {
    let first = bus_ids[0].1 as i128;

    //
    // First we will find the # bus cycles of the first bus before it arrives n
    // minutes before the other bus (where n is that bus' index) for each other bus. 
    // We can phrase this as a linear congruence and solve for the # of first bus cycles:
    //     - <first_bus_minutes> * x ≡ -<offset> (mod <other_bus_minutes>)
    //
    // We then know that this spacing will repeat every time LCM(first, other bus)
    // minutes pass a.k.a [LCM(first, other bus) / first] cycles of the first bus.
    //

    let relation_info: Vec<BusRelationInfo> = bus_ids[1..].iter().map(|&(offset, id)| {
        let lcm_minutes = lcm(first, id as i128);
        BusRelationInfo{
            first_bus_cycle_count: solve_linear_congruence(first, (-(offset as i128)).rem_euclid(id as i128), id as i128),
            subsequent_cycle_count: lcm_minutes / first,
        }
    }).collect();

    //
    // Now we will collapse our first cycle repeat values together using linear congruences
    // to find the overall solution to this part of the problem. For example, if:
    //     - The first bus will arrive the proper # of minutes before Bus_N for the first time
    //       after 19 first bus cycles (repeating that offset every 37 subsequent cycles)
    //     - The first bus will arrive the proper # of minutes before Bus_M for the first time
    //       after 970 first bus cycles (repeating that offset every 971 subsequent cycles)
    // we can write the following equation:
    //     - 37x ≡ (970 - 19) (mod 971)
    //
    // By solving for x, we know how many cycles it will take after the first 19 for these to line
    // up for the first time. We know that after that point, they will line up every LCM(37, 971)
    // cycles. That allows us to collapse 2 equations into one, which we can keep doing until we
    // only have 1 equation.
    //
    // We initialize this fold with (0, 1) as the accumulator, which represents:
    //     - The first bus will arrive the propr # of minutes before Bus_N for the first time
    //       after 0 first bus cycles (repeating that offset every 1 subsequent cycle)
    // as sort of an "identity"
    //

    let init = BusRelationInfo{first_bus_cycle_count: 0, subsequent_cycle_count: 1};
    let collapsed = relation_info.iter().fold(init, |acc, info| {
        let factor = acc.subsequent_cycle_count;
        let remainder = (info.first_bus_cycle_count - acc.first_bus_cycle_count).rem_euclid(info.subsequent_cycle_count);
        let modulus = info.subsequent_cycle_count;

        let solution = solve_linear_congruence(factor, remainder, modulus);
        BusRelationInfo{
            first_bus_cycle_count: acc.first_bus_cycle_count + acc.subsequent_cycle_count * solution,
            subsequent_cycle_count: lcm(acc.subsequent_cycle_count, info.subsequent_cycle_count),
        }
    });

    let earliest_timestamp = collapsed.first_bus_cycle_count * first;
    println!("[Part 2] Earliest timestamp satisfying all bus offsets: {}", earliest_timestamp);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let (earliest, bus_ids) = parse_input(&contents)?;

    part1(earliest, &bus_ids);
    part2(&bus_ids);
    Ok(())
}