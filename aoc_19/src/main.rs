#[macro_use] extern crate lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;
use util::{file::GenericParseError, res::Result};

#[derive(Clone)]
struct Rule {
    idx: usize,
    def: RuleDef,
}

#[derive(Clone)]
enum RuleDef {
    Character(char),
    OrSequence(Vec<Vec<usize>>),
}

impl std::str::FromStr for Rule {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new("^([0-9]+): (?:(\"[a-z]\")|([^\"].*))$").unwrap();
        }

        match RULE_RE.captures(s) {
            Some(caps) => {
                let idx = caps.get(1).unwrap().as_str().parse::<usize>()?;
                if let Some(ch) = caps.get(2) {
                    Ok(Rule{idx, def: RuleDef::Character(ch.as_str().chars().nth(1).unwrap())})
                } else {
                    let or_sequences = caps.get(3).unwrap().as_str().split("|").map(|s| {
                        s.split_whitespace().map(|n| n.parse::<usize>().map_err(|e| e.into()))
                        .collect::<std::result::Result<Vec<usize>, GenericParseError>>()
                    }).collect::<std::result::Result<Vec<Vec<usize>>, GenericParseError>>()?;

                    Ok(Rule{idx, def: RuleDef::OrSequence(or_sequences)})
                }
            },
            None => Err(GenericParseError::ValueError(format!("Invalid line: {}", s).to_owned()))
        }
    }
}

fn parse_input(contents: &String) -> Result<(HashMap<usize, Rule>, Vec<String>)> {
    let mut rules_out: HashMap<usize, Rule> = HashMap::new();
    let mut strings_out: Vec<String> = vec![];
    let mut parsing_rules: bool = true;
    for line in contents.lines() {
        if parsing_rules {
            if line == "" { parsing_rules = false; continue }
            let parsed_rule = line.parse::<Rule>()?;
            rules_out.insert(parsed_rule.idx, parsed_rule);
        } else {
            strings_out.push(line.to_owned());
        }
    }

    Ok((rules_out, strings_out))
}

struct RuleValueGetter {
    memoized: HashMap<usize, HashSet<String>>
}

impl RuleValueGetter {
    fn new() -> Self { RuleValueGetter{ memoized: HashMap::new() } }
    fn get_set_of_values_for_rule(&mut self, rules: &HashMap<usize, Rule>, rule_idx: usize) -> HashSet<String> {
        if let Some(mem) = self.memoized.get(&rule_idx) {
            return mem.clone()
        }

        let ret = match &rules.get(&rule_idx).unwrap().def {
            RuleDef::Character(c) => {
                vec![c.to_string()].into_iter().collect()
            },
            RuleDef::OrSequence(seqs) => {
                seqs.iter().flat_map(|seq| {
                    let first_rule = seq[0];
                    seq.iter().skip(1).fold(
                        self.get_set_of_values_for_rule(rules, first_rule),
                        |acc, next_idx| {
                            acc.iter().flat_map(|prefix| {
                                self.get_set_of_values_for_rule(rules, *next_idx).into_iter()
                                    .map(move |suffix| {
                                        let mut pre_copy = prefix.clone();
                                        pre_copy.push_str(&suffix);
                                        pre_copy
                                    })
                            }).collect::<HashSet<String>>()
                        }
                    ).into_iter()
                }).collect::<HashSet<String>>()
            },
        };

        self.memoized.insert(rule_idx, ret.clone());
        ret
    }
}

// Returns how much of str, starting at start_idx, matches the rule at rule index `to_check`
fn matches_rule(rules: &HashMap<usize, Rule>, to_check: usize, s: &String, start_idx: usize, d: usize) -> usize {
    match &rules.get(&to_check).unwrap().def {
        RuleDef::Character(c) => {
            if s.chars().nth(start_idx).unwrap() == *c { 1 } else { 0 }
        },
        RuleDef::OrSequence(seqs) => {
            seqs.iter().filter_map(|seq| {
                seq.iter().fold(Some(start_idx), |curr_str_idx, other_rule_idx| {
                    match curr_str_idx {
                        None => None,
                        Some(idx) if idx >= s.len() => None,
                        Some(idx) => {
                            match matches_rule(rules, *other_rule_idx, s, idx, d + 1) {
                                0 => None,
                                advance => Some(idx + advance),
                            }
                        },
                    }
                })
            }).nth(0).unwrap_or(start_idx) - start_idx
        }
    }
}

fn part1(rules: &HashMap<usize, Rule>, strings: &Vec<String>) {
    let num_full_match = strings.iter().filter(|&s| {
        matches_rule(rules, 0, s, 0, 0) == s.len()
    }).count();
    println!("[Part 1] Num that fully matched rule 0: {}", num_full_match);
}

/**
 * The initial common chain of rules:
 *     - 0: 8 | 11
 *     - 8: 42 | 42 8
 *     - 11: 42 31 | 42 11 31
 *
 * ends up condensing into the following pattern:
 *     [42]+ 42{N} 31{N}
 * where N >= 1.
 *
 * We will find all possible values for rules 42 and 31,
 * and then try to chunk input strings into segments matching those.
 *
 * It just so happens that all values for rules 42 and 31 are
 * of length 8, which is helpful for more efficient chunking.
 *
 * It also just so happens that the set of values that satisfy
 * rule 42 are disjoint from the set of values that satisfy rule
 * 31, which makes it even less complex to check if a string matches.
 */
fn part2(rules: &HashMap<usize, Rule>, strings: &Vec<String>) {
    let mut getter = RuleValueGetter::new();
    let vals42 = getter.get_set_of_values_for_rule(rules, 42);
    let vals31 = getter.get_set_of_values_for_rule(rules, 31);

    let mut num_matched = 0;
    for string in strings {
        let mut chunks: VecDeque<String> = string.chars().collect::<Vec<char>>()
            .chunks(8).map(|char_chunk| char_chunk.iter().collect::<String>())
            .collect::<VecDeque<String>>();

        // First, see what N is by repeatedly taking 8-char substrings from
        // the end of our string until they no longer match rule 31
        let mut n = 0;
        while chunks.len() > 2 {
            let last_eight = chunks.pop_back().unwrap();
            if vals31.contains(&last_eight) {
                n += 1;
            } else {
                chunks.push_back(last_eight); // Put it back, it doesn't match rule 31
                break
            }
        }

        if n == 0 { continue } // N needs to be >= 1

        // Next, try to match 8-char substrings from the end of our string
        // with a possible value for rule 42, N times
        let mut m = 0; // This should end up equal to N after the loop
        for _ in 0..n {
            match chunks.pop_back() {
                Some(last_eight) if vals42.contains(&last_eight) => m += 1,
                _ => break,
            }
        }

        if m != n { continue } // We couldn't satisfy the suffix of 42{N} 31{N}

        // Finally, all the remaining string chunks should match rule 42, and there should be at least 1
        if chunks.len() == 0 { continue }
        if chunks.iter().all(|chunk| vals42.contains(chunk)) { num_matched += 1; }
    }

    println!("[Part 2] Num that fully matched rule 0: {}", num_matched);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let (rules, strings) = parse_input(&contents)?;

    part1(&rules, &strings);
    part2(&rules, &strings);
    Ok(())
}