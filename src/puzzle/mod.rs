use std::path::{Path, PathBuf};

use crate::benchmark::RuntimeStats;

mod intcode;
pub mod template;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;

#[allow(clippy::type_complexity)]
pub struct Puzzle {
    puzzle: u32,
    input_file: PathBuf,
    p1: Box<dyn Fn(&str) -> Result<(RuntimeStats, String)>>,
    p2: Box<dyn Fn(&str) -> Result<(RuntimeStats, String)>>,
}

impl Puzzle {
    #[allow(clippy::type_complexity)]
    pub fn new(
        id: u32,
        input_file: impl AsRef<Path>,
        p1: Box<dyn Fn(&str) -> Result<(RuntimeStats, String)>>,
        p2: Box<dyn Fn(&str) -> Result<(RuntimeStats, String)>>,
    ) -> Self {
        Self {
            puzzle: id,
            input_file: input_file.as_ref().to_owned(),
            p1,
            p2,
        }
    }

    pub fn run(
        &self,
        parts: [bool; 2],
        mut visitor: impl FnMut(u32, u32, crate::benchmark::Result) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let input = std::fs::read_to_string(&self.input_file)?;

        if parts[0] {
            let result = (*self.p1)(input.as_str());
            visitor(self.puzzle, 1, result)?;
        }

        if parts[1] {
            let result = (*self.p2)(input.as_str());
            visitor(self.puzzle, 2, result)?;
        }

        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not implemented")]
    NotImplemented,
    #[error("{0} error: {1}")]
    String(String, String),
    #[error(transparent)]
    Intcode(#[from] intcode::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    fn execution(message: &str) -> Self {
        Self::String("Execution".to_string(), message.to_string())
    }

    fn input(message: &str) -> Self {
        Self::String("Input".to_string(), message.to_string())
    }

    fn search(message: &str) -> Self {
        Self::String("Search".to_string(), message.to_string())
    }

    fn parse(message: &str) -> Self {
        ParseError::String(message.to_string()).into()
    }
}

impl<T> From<nom::error::Error<T>> for Error
where
    T: std::fmt::Display,
{
    fn from(value: nom::error::Error<T>) -> Self {
        ParseError::from(value).into()
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseError::from(value).into()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Parse error: {0}")]
    String(String),
    #[error("{0}")]
    Nom(String),
    #[error(transparent)]
    Integer(#[from] std::num::ParseIntError),
}

impl<T> From<nom::error::Error<T>> for ParseError
where
    T: std::fmt::Display,
{
    fn from(value: nom::error::Error<T>) -> Self {
        Self::Nom(format!("{}", value))
    }
}
