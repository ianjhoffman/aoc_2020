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
    precedence: bool,
}

impl ExprParser {
    fn new(precedence: bool) -> ExprParser {
        ExprParser{ parse_stack: VecDeque::new(), precedence }
    }

    fn try_resolve_mul(substack: &VecDeque<Intermediate>) -> std::result::Result<Expr, GenericParseError> {
        let mut substack_clone = substack.clone();
        while substack_clone.len() > 1 {
            if substack_clone.len() < 3 {
                return Err(GenericParseError::ValueError("Fewer than 3 paren-internal intermediates".to_owned()));
            }

            // Try to turn first 3 elements, which should be of the form A * B, into C
            let first = substack_clone.pop_front();
            let second = substack_clone.pop_front();
            let third = substack_clone.pop_front();
            match (first, second, third) {
                (
                    Some(Intermediate::Resolved(o1)),
                    Some(Intermediate::Unresolved(Token::Mul)),
                    Some(Intermediate::Resolved(o2)),
                ) => {
                    substack_clone.push_front(Intermediate::Resolved(Expr::Mul(Box::new(o1), Box::new(o2))));
                },
                x => return Err(GenericParseError::ValueError(format!(
                    "Could not consolidate subexpression: {:?}", x
                ).to_owned())),
            }
        }

        match &substack_clone.iter().collect::<Vec<&Intermediate>>()[..] {
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
                self.try_resolve_expr(ExprParser::try_resolve_mul(&in_between)?)
            },
            f => return Err(GenericParseError::ValueError(
                format!("Unexpected closing paren succeeding [{:?}, {:?}]", f, in_between).to_owned()
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
                if self.precedence {
                    self.parse_stack.push_back(Intermediate::Resolved(o1));
                    self.parse_stack.push_back(Intermediate::Unresolved(Token::Mul));
                    self.parse_stack.push_back(Intermediate::Resolved(e));
                } else {
                    self.parse_stack.push_back(Intermediate::Resolved(Expr::Mul(Box::new(o1), Box::new(e))));
                }
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

        if self.precedence {
            let mul_resolved = Intermediate::Resolved(Self::try_resolve_mul(&self.parse_stack)?);
            self.parse_stack = vec![mul_resolved].into_iter().collect();
        }

        match &self.parse_stack.iter().collect::<Vec<&Intermediate>>()[..] {
            &[Intermediate::Resolved(e)] => Ok(e.clone()),
            _ => Err(GenericParseError::ValueError("Could not resolve expression!".to_owned()))
        }
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

fn part1(contents: &String) -> Result<()> {
    let expressions = parse_expressions(&contents, false)?;
    let eval_sum = expressions.iter().fold(0, |acc, e| acc + e.eval());
    Ok(println!("[Part 1] Sum of all evaluated expressions: {}", eval_sum))
}

fn part2(contents: &String) -> Result<()> {
    let expressions = parse_expressions(&contents, true)?;
    let eval_sum = expressions.iter().fold(0, |acc, e| acc + e.eval());
    Ok(println!("[Part 2] Sum of all evaluated expressions: {}", eval_sum))
}

fn parse_expressions(contents: &String, precedence: bool) -> std::result::Result<Vec<Expr>, GenericParseError> {
    contents.lines().map(|l| ExprParser::new(precedence).parse(tokenize(l)?)).collect()
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    part1(&contents)?;
    part2(&contents)
}