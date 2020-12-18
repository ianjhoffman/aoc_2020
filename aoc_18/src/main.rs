#[macro_use] extern crate lazy_static;
use std::collections::VecDeque;
use regex::Regex;
use util::res::Result;
use util::file::GenericParseError;

enum Token {
    LParen,
    RParen,
    Add,
    Mul,
    Num(i64),
}

fn tokenize(s: &str) -> std::result::Result<VecDeque<Token>, GenericParseError> {
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

#[derive(Clone)]
enum Expr {
    Val(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

struct ExprParser {
    tokens: VecDeque<Token>,
}

impl ExprParser {
    fn new(raw: &str) -> std::result::Result<ExprParser, GenericParseError> {
        Ok(ExprParser{ tokens: tokenize(raw)? })
    }

    fn parse(&mut self) -> std::result::Result<Expr, GenericParseError> {
        Ok(Expr::Val(0))
    }
}

impl std::str::FromStr for Expr {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        ExprParser::new(s)?.parse()
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