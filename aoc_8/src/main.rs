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
        match &s.split_whitespace().collect::<Vec<&str>>()[..] {
            ["nop", v] if v.starts_with(&['+', '-'][..]) => {
                Ok(Instruction::Nop(v.parse::<i64>()?))
            },
            ["acc", v] if v.starts_with(&['+', '-'][..]) => {
                Ok(Instruction::Acc(v.parse::<i64>()?))
            },
            ["jmp", v] if v.starts_with(&['+', '-'][..]) => {
                Ok(Instruction::Jmp(v.parse::<i64>()?))
            },
            _ => Err(GenericParseError::ValueError(format!("Invalid instruction: {}", s).to_owned()))
        }
    }
}

struct ProgramState {
    ip: usize,
    acc: i64,
}

impl ProgramState {
    fn eval(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Nop(_) => self.ip += 1,
            Instruction::Acc(v) => {
                self.acc += v;
                self.ip += 1;
            },
            Instruction::Jmp(v) => self.ip = (self.ip as i64 + *v) as usize,
        }
    }
}

struct BootCode {
    instructions: Vec<Instruction>,
}

impl BootCode {
    fn new(instructions: Vec<Instruction>) -> Self {
        BootCode{instructions: instructions}
    }

    fn eval_until_repeat_or_end(&self) -> (i64, bool) {
        let mut program_state = ProgramState{ip: 0, acc: 0};
        let mut seen_ips: HashSet<usize> = vec![0].into_iter().collect();
        loop {
            let curr_acc = program_state.acc;
            program_state.eval(&self.instructions[program_state.ip]);

            // Program repeated itself
            if !seen_ips.insert(program_state.ip) { return (curr_acc, false); }

            // Program terminated
            if program_state.ip == self.instructions.len() { return (program_state.acc, true); }
        }
    }
}

fn part1(instructions: &Vec<Instruction>) {
    let acc_value_before_first_repeat = BootCode::new(instructions.clone()).eval_until_repeat_or_end();
    println!("[Part 1] Value of `acc` before first IP repeat: {}", acc_value_before_first_repeat.0);
}

fn part2(instructions: &Vec<Instruction>) {
    let found = instructions.iter().enumerate().filter(|(_, instr)| {
        match instr {
            Instruction::Nop(_) | Instruction::Jmp(_) => true,
            _ => false,
        }
    }).map(|(idx, instr)| {
        let new_instr = match &instr {
            Instruction::Nop(v) => Instruction::Jmp(*v),
            Instruction::Jmp(v) => Instruction::Nop(*v),
            Instruction::Acc(v) => Instruction::Acc(*v), // Shouldn't ever happen
        };

        let mut modified_instructions = instructions.clone();
        modified_instructions[idx] = new_instr;
        BootCode::new(modified_instructions).eval_until_repeat_or_end()
    }).find(|(_, finished)| *finished);

    if let Some((acc, _)) = found {
        return println!("[Part 2] Value of `acc` after final instruction: {}", acc);
    }

    println!("[Part 2] Could not perform any swaps that resulted in program termination!")
}

fn main() -> Result<()> {
    let opt = Cli::from_args();
    let instructions: Vec<Instruction> = util::file::read_lines_to_type::<Instruction>(opt.file)?;

    part1(&instructions);
    part2(&instructions);
    Ok(())
}