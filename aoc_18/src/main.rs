#[macro_use] extern crate lazy_static;
use std::collections::{HashSet, VecDeque};
use regex::Regex;
use util::res::Result;
use util::file::GenericParseError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Binop {
    Add,
    Mul
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    LParen,
    RParen,
    Binop(Binop),
    Num(i64),
}

fn tokenize(s: &str) -> std::result::Result<Vec<Token>, GenericParseError> {
    lazy_static! {
        static ref TOKEN_REGEX: Regex = Regex::new(r"(\(|\)|\+|\*|[0-9]+)").unwrap();
    }

    TOKEN_REGEX.captures_iter(s).map(|m| {
        Ok(match &m[1] {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "+" => Token::Binop(Binop::Add),
            "*" => Token::Binop(Binop::Mul),
            n => Token::Num(n.parse::<i64>()?),
        })
    }).collect()
}

#[derive(Clone, Debug)]
enum Expr {
    Val(i64),
    Binop(Box<Expr>, Binop, Box<Expr>),
}

#[derive(Clone, Debug)]
enum Intermediate {
    Unresolved(Token),
    Resolved(Expr),
}

struct ExprParser {
    parse_stack: VecDeque<Intermediate>,
    precedence_rules: Vec<HashSet<Binop>>,
}

impl ExprParser {
    fn new(precedence_rules: Vec<HashSet<Binop>>) -> ExprParser {
        ExprParser{ parse_stack: VecDeque::new(), precedence_rules }
    }

    fn try_resolve_ops(&self, substack: &VecDeque<Intermediate>) -> std::result::Result<Expr, GenericParseError> {
        let mut processing = substack.clone();
        let mut processed: VecDeque<Intermediate> = VecDeque::new();
        for precedence_level in &self.precedence_rules {
            while processing.len() > 0 {
                // Try to turn first 3 elements, which should be of the form A <binop> B, into expr C
                let first = processing.pop_front();
                let second = processing.pop_front();
                let third = processing.pop_front();
                match (first.clone(), second.clone(), third.clone()) {
                    (Some(Intermediate::Resolved(r)), None, None) => {
                        processed.push_back(Intermediate::Resolved(r)); // Only one item left
                    },
                    (
                        Some(Intermediate::Resolved(o1)),
                        Some(Intermediate::Unresolved(Token::Binop(op))),
                        Some(Intermediate::Resolved(o2)),
                    ) => {
                        if precedence_level.contains(&op) {
                            processing.push_front(Intermediate::Resolved(Expr::Binop(Box::new(o1), op, Box::new(o2))));
                        } else {
                            // Wait to apply this op since it's not in the current precedence level
                            processed.push_back(first.unwrap());
                            processed.push_back(second.unwrap());
                            processing.push_front(third.unwrap()); // Keep 2nd operand to consider next
                        }
                    },
                    x => return Err(GenericParseError::ValueError(
                        format!("Could not resolve subexpression: [{:?}]", x).to_owned()
                    )),
                }
            }

            processing.clone_from(&processed.drain(..).collect::<VecDeque<Intermediate>>());
        }

        match &processing.iter().collect::<Vec<&Intermediate>>()[..] {
            &[Intermediate::Resolved(e)] => Ok(e.clone()),
            _ => Err(GenericParseError::ValueError("Could not resolve expression!".to_owned()))
        }
    }

    fn try_resolve_paren(&mut self) -> std::result::Result<(), GenericParseError> {
        // Search backwards until we find an unresolved left parentheses
        let mut in_between: VecDeque<Intermediate> = VecDeque::new();
        while self.parse_stack.len() > 0 {
            let prev = self.parse_stack.pop_back().unwrap();
            in_between.push_front(prev.clone());
            if let Intermediate::Unresolved(Token::LParen) = prev { break }
        }

        match in_between.pop_front() {
            Some(Intermediate::Unresolved(Token::LParen)) => {
                Ok({ self.parse_stack.push_back(
                    Intermediate::Resolved(self.try_resolve_ops(&in_between)?)
                )})
            },
            f => return Err(GenericParseError::ValueError(
                format!("Unexpected closing paren succeeding [{:?}, {:?}]", f, in_between).to_owned()
            )),
        }
    }

    fn parse(&mut self, tokens: Vec<Token>) -> std::result::Result<Expr, GenericParseError> {
        for token in tokens {
            match token {
                Token::LParen | Token::Binop(Binop::Add) | Token::Binop(Binop::Mul) => {
                    self.parse_stack.push_back(Intermediate::Unresolved(token))
                },
                Token::RParen => { self.try_resolve_paren()?; },
                Token::Num(v) => {
                    self.parse_stack.push_back(Intermediate::Resolved(Expr::Val(v)));
                },
            }
        }
        self.try_resolve_ops(&self.parse_stack)
    }
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Val(v) => *v,
            Expr::Binop(e1, op, e2) => match op {
                Binop::Add => e1.eval() + e2.eval(),
                Binop::Mul => e1.eval() * e2.eval(),
            },
        }
    }
}

fn part1(contents: &String) -> Result<()> {
    let expressions = parse_expressions(&contents, vec![
        vec![Binop::Add, Binop::Mul].into_iter().collect::<HashSet<Binop>>() // +/* have equal precedence
    ])?;
    let eval_sum = expressions.iter().fold(0, |acc, e| acc + e.eval());
    Ok(println!("[Part 1] Sum of all evaluated expressions: {}", eval_sum))
}

fn part2(contents: &String) -> Result<()> {
    let expressions = parse_expressions(&contents, vec![
        vec![Binop::Add].into_iter().collect::<HashSet<Binop>>(), // + has highest precedence
        vec![Binop::Mul].into_iter().collect::<HashSet<Binop>>(), // * has lower precedence
    ])?;
    let eval_sum = expressions.iter().fold(0, |acc, e| acc + e.eval());
    Ok(println!("[Part 2] Sum of all evaluated expressions: {}", eval_sum))
}

fn parse_expressions(contents: &String, precedence_rules: Vec<HashSet<Binop>>) -> std::result::Result<Vec<Expr>, GenericParseError> {
    contents.lines().map(|l| {
        ExprParser::new(precedence_rules.clone()).parse(tokenize(l)?)
    }).collect()
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    part1(&contents)?;
    part2(&contents)
}