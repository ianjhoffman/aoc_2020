#[macro_use] extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;
use util::res::Result;
use util::file::GenericParseError;

#[derive(Clone)]
struct BagRule {
    container_spec: String,
    contained_specs: HashMap<String, usize>,
}

impl std::str::FromStr for BagRule {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static!{
            static ref FULL_REGEX: Regex = Regex::new(
                r"^([a-z]+ [a-z]+) bags contain (?:(?:no other bags)|([0-9]+ .*)).$"
            ).unwrap();
            static ref CONTAINED_REGEX: Regex = Regex::new(
                r"([0-9]+)\s([a-z]+ [a-z]+) bags?"
            ).unwrap();
        }

        let full_line_caps = FULL_REGEX.captures(s).ok_or_else(
            || GenericParseError::ValueError(format!("Invalid line format: {}", s).to_owned())
        )?;

        let container_spec: String = full_line_caps.get(1).unwrap().as_str().to_owned();
        let mut contained_specs: HashMap<String, usize> = HashMap::new();
        if let Some(_) = full_line_caps.get(2) {
            for caps in CONTAINED_REGEX.captures_iter(s) {
                contained_specs.insert(
                    caps.get(2).unwrap().as_str().to_owned(),
                    caps.get(1).unwrap().as_str().parse::<usize>()?
                );
            }
        }

        Ok(BagRule{container_spec, contained_specs})
    }
}

fn bag_rules_to_container_mappings(bag_rules: &Vec<BagRule>) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for bag_rule in bag_rules {
        for (contained_spec, _) in &bag_rule.contained_specs {
            let out_entry = out.entry(contained_spec.clone()).or_insert(vec![]);
            out_entry.push(bag_rule.container_spec.clone());
        }
    }

    out
}

fn bag_rules_to_bag_rule_mappings(bag_rules: &Vec<BagRule>) -> HashMap<String, HashMap<String, usize>> {
    bag_rules.iter().cloned().map(|br| (br.container_spec, br.contained_specs)).collect()
}

fn part1(bag_rules: &Vec<BagRule>) {
    // BFS container mappings to find total bag count that could contain "shiny gold" bag
    let container_mappings = bag_rules_to_container_mappings(bag_rules);
    let mut seen: HashSet<String> = HashSet::new();
    let mut to_visit: VecDeque<String> = vec!["shiny gold".to_owned()]
        .into_iter().collect();

    while !to_visit.is_empty() {
        let visited = to_visit.pop_front().unwrap();
        if !seen.insert(visited.clone()) { continue; }
        container_mappings.get(&visited).unwrap_or(&vec![])
            .iter().for_each(|s| to_visit.push_back(s.clone()));
    }

    let num_possible_containers = seen.len() - 1; // Exclude "shiny gold"
    println!("[Part 1] Shiny gold bag could be in {} different types of bags!", num_possible_containers);
}

fn part2(bag_rules: &Vec<BagRule>) {
    // BFS bag rules to find total bags required within "shiny gold" bag
    let bag_rule_mappings = bag_rules_to_bag_rule_mappings(bag_rules);
    let mut count: usize = 0;

    // Keep track of a multiplier during our BFS that we will multiply by bag counts as we nest
    let mut to_visit: VecDeque<(String, usize)> = vec![("shiny gold".to_owned(), 1)]
        .into_iter().collect();
    
    while !to_visit.is_empty() {
        let (visited, multiplier) = to_visit.pop_front().unwrap();
        count += multiplier;
        bag_rule_mappings.get(&visited).unwrap_or(&HashMap::new())
            .iter().for_each(|(contained_spec, num_contained)| {
                to_visit.push_back((contained_spec.clone(), num_contained * multiplier));
            });
    }

    println!("[Part 2] Shiny gold bag must contain {} bags!", count - 1); // Exclude "shiny gold"
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let bag_rules: Vec<BagRule> = util::file::read_lines_to_type::<BagRule>(file_path)?;

    part1(&bag_rules);
    part2(&bag_rules);
    Ok(())
}