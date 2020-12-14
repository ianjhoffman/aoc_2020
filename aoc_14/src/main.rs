use std::collections::HashMap;
use util::res::Result;
use util::file::GenericParseError;

struct RawMask {
    zero: Vec<usize>,
    one: Vec<usize>,
    x: Vec<usize>,
}

impl RawMask {
    fn to_v1(&self) -> MaskV1 {
        let and = self.zero.iter().fold(std::u64::MAX, |acc, shift| acc ^ (1 << shift));
        let or = self.one.iter().fold(0, |acc, shift| acc ^ (1 << shift));
        MaskV1{and, or}
    }

    fn to_v2(&self) -> MaskV2 {
        let or = self.one.iter().fold(0, |acc, shift| acc ^ (1 << shift));
        let floating_and_mask = self.x.iter().fold(std::u64::MAX, |acc, shift| acc ^ (1 << shift));

        // Find all possible values for floating bits
        let mut floating_possibility_or_masks = vec![0];
        for shift in &self.x {
            floating_possibility_or_masks = floating_possibility_or_masks.into_iter().flat_map(|incomplete| {
                vec![incomplete, incomplete | (1 << shift)].into_iter()
            }).collect();
        }

        MaskV2{or, floating_and_mask, floating_possibility_or_masks}
    }
}

// Version 1 mask, for Part 1

struct MaskV1 {
    and: u64, // For overwriting with zeroes
    or: u64, // For overwriting with ones
}

impl MaskV1 {
    fn apply(&self, val: u64) -> u64 {
        (val & self.and) | self.or
    }
}

// Version 2 mask, for Part 2

struct MaskV2 {
    or: u64, // For overwriting with ones
    floating_and_mask: u64, // For clearing out floating bits before trying another possibility
    floating_possibility_or_masks: Vec<u64>,
}

impl MaskV2 {
    fn apply(&self, val: u64) -> Vec<u64> {
        self.floating_possibility_or_masks.iter().map(|m| ((val | self.or) & self.floating_and_mask) | m).collect()
    }
}

enum Instruction {
    Mem(u64, u64),
    Mask(RawMask),
}

impl std::str::FromStr for Instruction {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &s.split(" = ").collect::<Vec<&str>>()[..] {
            &["mask", mask_str] => {
                let (mut zero, mut one, mut x) = (vec![], vec![], vec![]);
                mask_str.chars().rev().enumerate().for_each(
                    |(idx, c)| {
                        match c {
                            '1' => one.push(idx),
                            '0' => zero.push(idx),
                            _ => x.push(idx),
                        }
                    },
                );
                Ok(Instruction::Mask(RawMask{zero, one, x}))
            },
            &[mem_addr_str, mem_val_str] => {
                let mem_addr = mem_addr_str.strip_prefix("mem[").and_then(|rem| rem.strip_suffix("]"))
                    .unwrap_or("-").parse::<u64>()?;
                let mem_value = mem_val_str.parse::<u64>()?;
                Ok(Instruction::Mem(mem_addr, mem_value))
            },
            _ => return Err(GenericParseError::ValueError(format!("Invalid instruction: {}", s).to_owned())),
        }
    }
}

fn part1(instructions: &Vec<Instruction>) {
    let mut curr_mask = MaskV1{and: std::u64::MAX, or: 0};
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => curr_mask = m.to_v1(),
            Instruction::Mem(addr, val) => { mem.insert(*addr, curr_mask.apply(*val)); },
        }
    }
     
    let memory_sum: u64 = mem.values().sum();
    println!("[Part 1] Sum of all values in memory after instructions complete: {}", memory_sum);
}

fn part2(instructions: &Vec<Instruction>) {
    let mut curr_mask = MaskV2{or: 0, floating_and_mask: std::u64::MAX, floating_possibility_or_masks: vec![]};
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => curr_mask = m.to_v2(),
            Instruction::Mem(addr, val) => {
                curr_mask.apply(*addr).into_iter().for_each(|new_addr| { mem.insert(new_addr, *val); });
            },
        }
    }

    let memory_sum: u64 = mem.values().sum();
    println!("[Part 2] Sum of all values in memory after instructions complete: {}", memory_sum);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let instructions = util::file::read_lines_to_type::<Instruction>(file_path)?;

    part1(&instructions);
    part2(&instructions);
    Ok(())
}