#[macro_use] extern crate lazy_static;

use regex::Regex;
use util::res::Result;

struct Policy {
    letter: char,
    lower: usize,
    upper: usize,
}

impl Policy {
    fn validate_range(&self, password: &String) -> bool {
        let char_count = password.chars().filter(|c| *c == self.letter).count();
        char_count >= self.lower && char_count <= self.upper
    }

    fn validate_no_duplicate(&self, password: &String) -> bool {
        let chars: Vec<char> = password.chars().collect();
        (chars[self.lower - 1] == self.letter) ^ (chars[self.upper - 1] == self.letter)
    }
}

struct PolicyAndPassword {
    policy: Policy,
    password: String,
}

impl std::str::FromStr for PolicyAndPassword {
    type Err = util::file::GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();
        }
        
        if let Some(caps) = LINE_REGEX.captures(s) {
            let policy = Policy{
                letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                lower: caps.get(1).unwrap().as_str().parse::<usize>()?,
                upper: caps.get(2).unwrap().as_str().parse::<usize>()?,
            };

            return Ok(PolicyAndPassword{
                policy: policy,
                password: caps.get(4).unwrap().as_str().to_owned(),
            })
        }

        Err(util::file::GenericParseError::ValueError(format!("Invalid line: {}", s).to_owned()))
    }
}

fn part1(policies_and_passwords: &Vec<PolicyAndPassword>) {
    let num_valid = policies_and_passwords.iter()
        .filter(|p| p.policy.validate_range(&p.password)).count();
    
    println!("[Part 1] {} / {} passwords are valid!", num_valid, policies_and_passwords.len());
}

fn part2(policies_and_passwords: &Vec<PolicyAndPassword>) {
    let num_valid = policies_and_passwords.iter()
    .filter(|p| p.policy.validate_no_duplicate(&p.password)).count();

    println!("[Part 2] {} / {} passwords are valid!", num_valid, policies_and_passwords.len());
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let policies_and_passwords: Vec<PolicyAndPassword> = util::file::read_lines_to_type::<PolicyAndPassword>(file_path)?;

    part1(&policies_and_passwords);
    part2(&policies_and_passwords);
    Ok(())
}