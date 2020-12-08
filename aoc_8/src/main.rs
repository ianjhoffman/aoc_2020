use std::path::PathBuf;
use std::collections::HashSet;
use util::res::Result;
use util::file::GenericParseError;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl std::str::FromStr for Instruction {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let e = Err(GenericParseError::ValueError(format!("Invalid instruction: {}", s).to_owned()));
        Ok(match &s.split_whitespace().collect::<Vec<&str>>()[..] {
            [name, v] if v.starts_with(&['+', '-'][..]) => {
                match *name {
                    "nop" => Instruction::Nop(v.parse::<i64>()?),
                    "acc" => Instruction::Acc(v.parse::<i64>()?),
                    "jmp" => Instruction::Jmp(v.parse::<i64>()?),
                    _ => return e,
                }
            },
            _ => return e,
        })
    }
}

fn eval_until_repeat_or_end(instructions: &Vec<Instruction>) -> (i64, bool) {
    let (mut ip, mut acc): (usize, i64) = (0, 0);
    let mut seen_ips: HashSet<usize> = vec![0].into_iter().collect();
    loop {
        let (new_ip, new_acc) = match instructions[ip] {
            Instruction::Nop(_) => (ip + 1, acc),
            Instruction::Acc(v) => (ip + 1, acc + v),
            Instruction::Jmp(v) => ((v + ip as i64) as usize, acc),
        };

        // Program repeated itself
        if !seen_ips.insert(new_ip) { return (acc, false); }

        // Program terminated
        if new_ip == instructions.len() { return (new_acc, true); }

        ip = new_ip;
        acc = new_acc;
    }
}

fn part1(instructions: &Vec<Instruction>) {
    let acc_value_before_first_repeat = eval_until_repeat_or_end(instructions);
    println!("[Part 1] Value of `acc` before first IP repeat: {}", acc_value_before_first_repeat.0);
}

fn part2(instructions: &Vec<Instruction>) {
    match instructions.iter().enumerate().filter_map(|(idx, instr)| {
        match instr {
            Instruction::Nop(v) => Some((idx, Instruction::Jmp(*v))),
            Instruction::Jmp(v) => Some((idx, Instruction::Nop(*v))),
            Instruction::Acc(_) => None,
        }
    }).map(|(idx, new_instr)| {
        let mut modified_instructions = instructions.clone();
        modified_instructions[idx] = new_instr;
        eval_until_repeat_or_end(&modified_instructions)
    }).find(|(_, finished)| *finished) {
        Some((acc, _)) => println!("[Part 2] Value of `acc` after final instruction: {}", acc),
        None => println!("[Part 2] Could not perform any swaps that resulted in program termination!")
    }
}

fn main() -> Result<()> {
    let opt = Cli::from_args();
    let instructions: Vec<Instruction> = util::file::read_lines_to_type::<Instruction>(opt.file)?;

    part1(&instructions);
    part2(&instructions);
    Ok(())
}