#[macro_use] extern crate lazy_static;

use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use util::res::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

#[derive(Clone, Copy)]
enum HeightUnit {
    Inch,
    Cm,
}

enum ParsedField<'a> {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(Option<(u32, HeightUnit)>),
    HairColor(&'a str),
    EyeColor(&'a str),
    PassportID(&'a str),
    CountryID(&'a str),
}

impl<'a> ParsedField<'a> {
    fn from_key_and_value(key: &'a str, value: &'a str) -> Result<ParsedField<'a>> {
        match key {
            "byr" => Ok(ParsedField::BirthYear(value.parse::<u32>()?)),
            "iyr" => Ok(ParsedField::IssueYear(value.parse::<u32>()?)),
            "eyr" => Ok(ParsedField::ExpirationYear(value.parse::<u32>()?)),
            "hgt" => {
                for (unit_str, typed) in &[("in", HeightUnit::Inch), ("cm", HeightUnit::Cm)] {
                    if let Some(num_str) = value.strip_suffix(unit_str) {
                        return Ok(ParsedField::Height(Some((num_str.parse::<u32>()?, *typed))));
                    }
                }

                Ok(ParsedField::Height(None))
            },
            "hcl" => Ok(ParsedField::HairColor(value)),
            "ecl" => Ok(ParsedField::EyeColor(value)),
            "pid" => Ok(ParsedField::PassportID(value)),
            "cid" => Ok(ParsedField::CountryID(value)),
            _ => Err(From::from(format!("Invalid passport field key: {}", key)))
        }
    }

    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref VALID_EYE_COLORS: HashSet<&'static str> = [
                "amb", "blu", "brn", "gry", "grn", "hzl", "oth",
            ].iter().map(|f| *f).collect::<HashSet<&'static str>>();
        }

        match self {
            ParsedField::BirthYear(byr) => *byr >= 1920 && *byr <= 2002,
            ParsedField::IssueYear(iyr) => *iyr >= 2010 && *iyr <= 2020,
            ParsedField::ExpirationYear(eyr) => *eyr >= 2020 && *eyr <= 2030,
            ParsedField::Height(Some((val, unit))) => match unit {
                HeightUnit::Inch => *val >= 59 && *val <= 76,
                HeightUnit::Cm => *val >= 150 && *val <= 193,
            },
            ParsedField::Height(None) => false,
            ParsedField::HairColor(hcl) => {
                hcl.len() == 7
                && hcl.chars().nth(0).unwrap() == '#'
                && hcl.chars().skip(1).all(|c| c.is_alphanumeric())
            },
            ParsedField::EyeColor(ecl) => VALID_EYE_COLORS.contains(*ecl),
            ParsedField::PassportID(pid) => pid.len() == 9 && pid.parse::<u32>().is_ok(),
            ParsedField::CountryID(_) => true, // Ignored
        }
    }
}

struct Passport<'a> {
    present_fields: HashSet<&'a str>,
    parsed_fields: Vec<ParsedField<'a>>,
}

impl<'a> Passport<'a> {
    fn is_valid(&self, keys_only: bool) -> bool {
        lazy_static! {
            static ref REQUIRED_FIELDS: HashSet<&'static str> = [
                "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
            ].iter().map(|f| *f).collect::<HashSet<&'static str>>();
        };
        
        if self.present_fields.intersection(&REQUIRED_FIELDS).count() != REQUIRED_FIELDS.len() {
            return false;
        }
        keys_only || self.parsed_fields.iter().all(|pf| pf.is_valid())
    }
}

fn file_contents_to_passports(contents: &String) -> Result<Vec<Passport>> {
    let mut out = vec![];
    
    let mut curr_field_values: HashMap<&str, &str> = HashMap::new();

    // Add an extra blank line to the end of the lines so we don't have to do extra post-loop logic
    for line in contents.lines().chain(std::iter::once("")) {
        if line == "" {
            out.push(Passport{
                present_fields: curr_field_values.keys().map(|k| *k).collect::<HashSet<&str>>(),
                parsed_fields: curr_field_values.iter()
                    .map(|(k, v)| ParsedField::from_key_and_value(k, v))
                    .collect::<Result<Vec<ParsedField>>>()?,
            });

            curr_field_values.clear();
        }

        for kv in line.split_whitespace() {
            let key_value_vec = kv.split(":").collect::<Vec<&str>>();
            if key_value_vec.len() != 2 {
                return Err(From::from(format!("Invalid password K/V pair: {}", kv)));
            }

            curr_field_values.insert(key_value_vec[0], key_value_vec[1]);
        }
    }

    Ok(out)
}

fn get_num_valid_passports(passports: &Vec<Passport>, keys_only: bool) -> usize {
    passports.iter().filter(|p| p.is_valid(keys_only)).count()
}

fn part1(passports: &Vec<Passport>) {
    println!("[Part 1] {} / {} passports are valid", get_num_valid_passports(passports, true), passports.len());
}

fn part2(passports: &Vec<Passport>) {
    println!("[Part 1] {} / {} passports are valid", get_num_valid_passports(passports, false), passports.len());
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let contents = util::file::read_to_string(opt.file)?;
    let passports = file_contents_to_passports(&contents)?;

    part1(&passports);
    part2(&passports);
    Ok(())
}