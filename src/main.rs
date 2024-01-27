use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::{
    benchmark::{measure, DurationFormatter},
    puzzle::Puzzle,
    report::Report,
};

mod benchmark;
mod puzzle;
mod report;
mod util;

#[derive(Parser)]
struct Args {
    /// Optional puzzle to run
    puzzle: Option<u32>,
    /// Optional part to run
    #[arg(short, long)]
    part: Option<u32>,
    /// Benchmarking rounds
    #[arg(short = 'r', long = "rounds", default_value_t = 1)]
    rounds: u32,
    /// Optional benchmark report output location
    #[arg(short = 'o', long = "out", id = "PATH")]
    report: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    trace();

    let args = Args::parse();
    let parts = match args.part {
        Some(1) => [true, false],
        Some(2) => [false, true],
        None => [true, true],
        _ => [false, false],
    };

    let rounds = args.rounds;

    let puzzles = [
        Puzzle::new(
            0,
            puzzle::template::INPUT_FILE,
            measure(puzzle::template::part1, rounds),
            measure(puzzle::template::part2, rounds),
        ),
        Puzzle::new(
            1,
            puzzle::day01::INPUT_FILE,
            measure(puzzle::day01::part1, rounds),
            measure(puzzle::day01::part2, rounds),
        ),
        Puzzle::new(
            2,
            puzzle::day02::INPUT_FILE,
            measure(puzzle::day02::part1, rounds),
            measure(puzzle::day02::part2, rounds),
        ),
        Puzzle::new(
            3,
            puzzle::day03::INPUT_FILE,
            measure(puzzle::day03::part1, rounds),
            measure(puzzle::day03::part2, rounds),
        ),
        Puzzle::new(
            4,
            puzzle::day04::INPUT_FILE,
            measure(puzzle::day04::part1, rounds),
            measure(puzzle::day04::part2, rounds),
        ),
        Puzzle::new(
            5,
            puzzle::day05::INPUT_FILE,
            measure(puzzle::day05::part1, rounds),
            measure(puzzle::day05::part2, rounds),
        ),
        Puzzle::new(
            6,
            puzzle::day06::INPUT_FILE,
            measure(puzzle::day06::part1, rounds),
            measure(puzzle::day06::part2, rounds),
        ),
        Puzzle::new(
            7,
            puzzle::day07::INPUT_FILE,
            measure(puzzle::day07::part1, rounds),
            measure(puzzle::day07::part2, rounds),
        ),
        Puzzle::new(
            8,
            puzzle::day08::INPUT_FILE,
            measure(puzzle::day08::part1, rounds),
            measure(puzzle::day08::part2, rounds),
        ),
        Puzzle::new(
            9,
            puzzle::day09::INPUT_FILE,
            measure(puzzle::day09::part1, rounds),
            measure(puzzle::day09::part2, rounds),
        ),
        Puzzle::new(
            10,
            puzzle::day10::INPUT_FILE,
            measure(puzzle::day10::part1, rounds),
            measure(puzzle::day10::part2, rounds),
        ),
        Puzzle::new(
            11,
            puzzle::day11::INPUT_FILE,
            measure(puzzle::day11::part1, rounds),
            measure(puzzle::day11::part2, rounds),
        ),
        Puzzle::new(
            12,
            puzzle::day12::INPUT_FILE,
            measure(puzzle::day12::part1, rounds),
            measure(puzzle::day12::part2, rounds),
        ),
        Puzzle::new(
            13,
            puzzle::day13::INPUT_FILE,
            measure(puzzle::day13::part1, rounds),
            measure(puzzle::day13::part2, rounds),
        ),
        Puzzle::new(
            14,
            puzzle::day14::INPUT_FILE,
            measure(puzzle::day14::part1, rounds),
            measure(puzzle::day14::part2, rounds),
        ),
        Puzzle::new(
            15,
            puzzle::day15::INPUT_FILE,
            measure(puzzle::day15::part1, rounds),
            measure(puzzle::day15::part2, rounds),
        ),
        Puzzle::new(
            16,
            puzzle::day16::INPUT_FILE,
            measure(puzzle::day16::part1, rounds),
            measure(puzzle::day16::part2, rounds),
        ),
    ];

    let start = Instant::now();

    let mut report = args.report.as_ref().map(|_| Report::default());

    let mut sum_of_medians = Duration::ZERO;
    let visitor = |puzzle, part, result: benchmark::Result| {
        match result {
            Ok((stats, result)) => {
                println!("Day {puzzle:02} part {part} ({stats}): {result}");
                sum_of_medians += stats.median();

                if let Some(report) = report.as_mut() {
                    report.push_entry(puzzle, part, &stats);
                }
            }
            Err(err) => {
                println!("Day {puzzle:02} part {part}: {err}");
            }
        }
        Ok(())
    };

    if let Some(puzzle) = args.puzzle {
        run_one(puzzle, &puzzles, parts, visitor)?;
    } else {
        run_all(&puzzles, parts, visitor)?;
    }

    let total = start.elapsed();

    if rounds > 1 {
        println!(
            "Sum of median solve times: {}",
            DurationFormatter(sum_of_medians),
        );
    } else {
        println!("Sum of solve times: {}", DurationFormatter(sum_of_medians),);
    }

    println!("Total time: {}", DurationFormatter(total));

    if let Some(report) = report {
        report.save_to(args.report.unwrap())?;
    }

    Ok(())
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No puzzle {puzzle}")]
    NoSuchPuzzle { puzzle: u32 },
    #[error(transparent)]
    Puzzle(#[from] puzzle::Error),
    #[error(transparent)]
    Report(csv::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        match value.kind() {
            csv::ErrorKind::Io(_) => {
                let io = match value.into_kind() {
                    csv::ErrorKind::Io(io) => io,
                    _ => unreachable!(),
                };

                Self::Io(io)
            }
            _ => Self::Report(value),
        }
    }
}

fn run_all(
    puzzles: &[Puzzle],
    parts: [bool; 2],
    mut visitor: impl FnMut(u32, u32, benchmark::Result) -> Result<()>,
) -> Result<()> {
    for puzzle in puzzles[1..].iter() {
        puzzle.run(parts, &mut visitor)?;
    }

    Ok(())
}

fn run_one(
    puzzle: u32,
    puzzles: &[Puzzle],
    parts: [bool; 2],
    visitor: impl FnMut(u32, u32, benchmark::Result) -> Result<()>,
) -> Result<()> {
    let puzzle = puzzles
        .get(puzzle as usize)
        .ok_or(Error::NoSuchPuzzle { puzzle })?;

    puzzle.run(parts, visitor)
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC_LOG"))
        .init();
}
