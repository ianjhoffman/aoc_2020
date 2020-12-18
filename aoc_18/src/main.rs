#[macro_use] extern crate lazy_static;
use std::collections::VecDeque;
use regex::Regex;
use util::res::Result;
use util::file::GenericParseError;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    LParen,
    RParen,
    Add,
    Mul,
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
            "+" => Token::Add,
            "*" => Token::Mul,
            n => Token::Num(n.parse::<i64>()?),
        })
    }).collect()
}

#[derive(Clone, Debug)]
enum Expr {
    Val(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug)]
enum Intermediate {
    Unresolved(Token),
    Resolved(Expr),
}

struct ExprParser {
    parse_stack: VecDeque<Intermediate>,
}

impl ExprParser {
    fn new() -> ExprParser {
        ExprParser{ parse_stack: VecDeque::new() }
    }

    fn try_resolve_paren(&mut self) -> std::result::Result<(), GenericParseError> {
        let prev = self.parse_stack.pop_back();
        let prev_prev = self.parse_stack.pop_back();
        match (prev_prev, prev) {
            (Some(Intermediate::Unresolved(Token::LParen)), Some(Intermediate::Resolved(e))) => {
                self.try_resolve_expr(e)
            },
            (pp, p) => Err(GenericParseError::ValueError(
                format!("Unexpected closing paren succeeding [{:?}, {:?}]", pp, p).to_owned()
            )),
        }
    }

    fn try_resolve_expr(&mut self, e: Expr) -> std::result::Result<(), GenericParseError> {
        let prev = self.parse_stack.pop_back();
        let prev_prev = self.parse_stack.pop_back();
        match (prev_prev, prev) {
            (None, None) => {
                self.parse_stack.push_back(Intermediate::Resolved(e));
                Ok(()) // This is the first token
            },
            (pp, Some(Intermediate::Unresolved(Token::LParen))) => {
                pp.and_then(|i| { self.parse_stack.push_back(i); Some(()) });
                self.parse_stack.push_back(Intermediate::Unresolved(Token::LParen));
                self.parse_stack.push_back(Intermediate::Resolved(e));
                Ok(()) // This is the beginning of a parenthetical
            },
            (Some(Intermediate::Resolved(o1)), Some(Intermediate::Unresolved(Token::Add))) => {
                self.parse_stack.push_back(Intermediate::Resolved(Expr::Add(
                    Box::new(o1), Box::new(e)
                )));
                Ok(()) // This is the 2nd operand of an addition
            },
            (Some(Intermediate::Resolved(o1)), Some(Intermediate::Unresolved(Token::Mul))) => {
                self.parse_stack.push_back(Intermediate::Resolved(Expr::Mul(
                    Box::new(o1), Box::new(e)
                )));
                Ok(()) // This is the 2nd operand of a multiplication
            },
            _ => Err(GenericParseError::ValueError(format!("Unexpected expr: {:?}", e).to_owned()))
        }
    }

    fn parse(&mut self, tokens: Vec<Token>) -> std::result::Result<Expr, GenericParseError> {
        for token in tokens {
            match token {
                Token::LParen | Token::Add | Token::Mul => {
                    self.parse_stack.push_back(Intermediate::Unresolved(token))
                },
                Token::RParen => { self.try_resolve_paren()?; },
                Token::Num(v) => { self.try_resolve_expr(Expr::Val(v.clone()))?; },
            }
        }

        match &self.parse_stack.iter().collect::<Vec<&Intermediate>>()[..] {
            &[Intermediate::Resolved(e)] => Ok(e.clone()),
            _ => Err(GenericParseError::ValueError("Could not resolve expression!".to_owned()))
        }
    }
}

impl std::str::FromStr for Expr {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        ExprParser::new().parse(tokenize(s)?)
    }
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Val(v) => *v,
            Expr::Add(e1, e2) => e1.eval() + e2.eval(),
            Expr::Mul(e1, e2) => e1.eval() * e2.eval(),
        }
    }
}

fn part1(expressions: &Vec<Expr>) {
    let eval_sum = expressions.iter().fold(0, |acc, e| acc + e.eval());
    println!("[Part 1] Sum of all evaluated expressions: {}", eval_sum);
}

fn part2(expressions: &Vec<Expr>) {
    
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let expressions = util::file::read_lines_to_type::<Expr>(file_path)?;

    part1(&expressions);
    part2(&expressions);
    Ok(())
}