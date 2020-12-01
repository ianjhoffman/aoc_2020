use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use super::res::Result;

pub fn read_to_string(path: PathBuf) -> Result<String> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug)]
pub enum GenericParseError {
    LineError,
    ValueError(String),
}

impl fmt::Display for GenericParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self {
            GenericParseError::LineError => "Could not successfully read line".to_owned(),
            GenericParseError::ValueError(m) => format!("Parse failed: {}", m),
        };

        write!(f, "{}", err_msg)
    }
}

impl From<std::io::Error> for GenericParseError {
    fn from(_: std::io::Error) -> Self {
        GenericParseError::LineError
    }
}

impl From<std::num::ParseIntError> for GenericParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        GenericParseError::ValueError(error.to_string())
    }
}

impl std::error::Error for GenericParseError {}

// To use read_lines_to_type, make sure your struct implements FromStr with:
// type Err = GenericParseError;
pub fn read_lines_to_type<T: FromStr<Err = GenericParseError>>(path: PathBuf) -> Result<Vec<T>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    reader.lines().map(|line| {
        line.map_or(Err(GenericParseError::LineError), |l| l.parse::<T>()).map_err(|e| e.into())
    }).collect()
}