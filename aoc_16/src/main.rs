#[macro_use] extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use regex::Regex;
use util::file::GenericParseError;
use util::res::Result;

struct Constraint {
    name: String,
    ranges: Vec<std::ops::RangeInclusive<u64>>,
}

impl std::str::FromStr for Constraint {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref CONSTRAINT_REGEX: Regex = Regex::new(
                r"^(?P<name>[^:]+): (?P<lower1>[0-9]+)-(?P<upper1>[0-9]+) or (?P<lower2>[0-9]+)-(?P<upper2>[0-9]+)$"
            ).unwrap();
        }

        if let Some(caps) = CONSTRAINT_REGEX.captures(s) {
            let lower1 = caps.name("lower1").unwrap().as_str().parse::<u64>()?;
            let upper1 = caps.name("upper1").unwrap().as_str().parse::<u64>()?;
            let lower2 = caps.name("lower2").unwrap().as_str().parse::<u64>()?;
            let upper2 = caps.name("upper2").unwrap().as_str().parse::<u64>()?;
            Ok(Constraint{
                name: caps.name("name").unwrap().as_str().to_owned(),
                ranges: vec![(lower1..=upper1), (lower2..=upper2)],
            })
        } else {
            Err(GenericParseError::ValueError(format!("Invalid constraint: {}", s).to_owned()))
        }
    }
}

impl Constraint {
    fn check(&self, val: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&val))
    }
}

struct Ticket {
    fields: Vec<u64>,
}

impl std::str::FromStr for Ticket {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Ticket{
            fields: s.split(",").map(|n| {
                n.parse::<u64>().map_err(|e| e.into())
            }).collect::<std::result::Result<Vec<u64>, GenericParseError>>()?,
        })
    }
}

impl Ticket {
    fn get_out_of_range_count(&self, constraints: &Vec<Constraint>) -> Option<u64> {
        self.fields.iter().fold(None, |acc, f| {
            match (acc, constraints.iter().any(|c| c.check(*f))) {
                (x, true) => x,
                (None, false) => Some(*f),
                (Some(s), false) => Some(s + *f),
            }
        })
    }
}

struct Input {
    constraints: Vec<Constraint>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

enum ParseState {
    Constraints,
    YourTicket,
    NearbyTickets,
}

impl Input {
    fn from_contents(contents: String) -> Result<Input> {
        let mut state = ParseState::Constraints;
        let mut out = Input{
            constraints: vec![],
            your_ticket: Ticket{ fields: vec![] },
            nearby_tickets: vec![],
        };

        for line in contents.lines() {
            if line == "" { continue }
            match state {
                ParseState::Constraints => {
                    match line {
                        "your ticket:" => state = ParseState::YourTicket,
                        l => out.constraints.push(l.parse::<Constraint>()?),
                    }
                },
                ParseState::YourTicket => {
                    match line {
                        "nearby tickets:" => state = ParseState::NearbyTickets,
                        l => out.your_ticket = l.parse::<Ticket>()?,
                    }
                },
                ParseState::NearbyTickets => out.nearby_tickets.push(line.parse::<Ticket>()?),
            }
        }
        
        Ok(out)
    }
}

fn part1(input: &Input) {
    let error_rate = input.nearby_tickets.iter().fold(0u64, |acc, t| {
        acc + t.get_out_of_range_count(&input.constraints).unwrap_or(0)
    });

    println!("[Part 1] Ticket scanning error rate: {}", error_rate);
}

fn part2(input: &Input) {
    let remaining_tickets: Vec<&Ticket> = input.nearby_tickets.iter()
        .filter(|t| t.get_out_of_range_count(&input.constraints) == None).collect();
    
    // Keep track of which ticket field indices are still all valid for a given constraint index
    let mut valid_field_indices_per_constraint: HashMap<usize, Vec<bool>> = (0..input.constraints.len())
        .map(|idx| (idx, vec![true; remaining_tickets[0].fields.len()])).collect();

    for remaining_ticket in remaining_tickets {
        for (constraint_idx, constraint) in input.constraints.iter().enumerate() {
            for (field_idx, field) in remaining_ticket.fields.iter().enumerate() {
                valid_field_indices_per_constraint.entry(constraint_idx)
                    .and_modify(|v| v[field_idx] &= constraint.check(*field));
            }
        }
    }

    // Turn into set of possible valid indices per constraint
    let mut valid_field_index_set_by_constraint: HashMap<usize, HashSet<usize>> =
        valid_field_indices_per_constraint.iter().map(|(k, v)| {(
                *k,
                v.iter().enumerate().filter(|(_, b)| **b).map(|(idx, _)| idx)
                    .collect::<HashSet<usize>>(),
        )}).collect();
    
    // Find mapping from field name to ticket index by process of elimination
    let mut field_name_to_index: HashMap<String, usize> = HashMap::new();
    while field_name_to_index.len() < input.constraints.len() {
        // Find constraint with only one possible ticket index
        let (constraint_idx, field_idx) = {
            let (c_idx, f_idx_set) = valid_field_index_set_by_constraint.iter()
                .find(|(_, v)| v.len() == 1).unwrap();

            (*c_idx, *f_idx_set.iter().nth(0).unwrap())
        };
        
        // Add that field name -> ticket index mapping
        field_name_to_index.insert(input.constraints[constraint_idx].name.clone(), field_idx);

        // Remove this entry for future iterations
        valid_field_index_set_by_constraint.remove(&constraint_idx);

        // Remove this field_idx from all other constraints' index sets
        valid_field_index_set_by_constraint.iter_mut().for_each(|(_, s)| { s.remove(&field_idx); });
    }

    // Find the product of the fields starting with "departure" on your ticket
    let product = field_name_to_index.iter().filter(|(k, _)| k.starts_with("departure"))
        .fold(1, |acc, (_, idx)| acc * input.your_ticket.fields[*idx]);
    
    println!("[Part 2] Product of 6 departure fields: {}", product);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let input = Input::from_contents(util::file::read_to_string(file_path)?)?;

    part1(&input);
    part2(&input);
    Ok(())
}