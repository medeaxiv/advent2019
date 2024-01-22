use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use benchmark::RuntimeStats;
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
    ];

    let start = Instant::now();

    let mut report = args.report.as_ref().map(|_| Report::default());

    let mut sum_of_medians = Duration::ZERO;
    let visitor = |puzzle, part, stats: RuntimeStats, result| {
        println!("Day {puzzle:02} part {part} ({stats}): {result}");
        sum_of_medians += stats.median();

        if let Some(report) = report.as_mut() {
            report.push_entry(puzzle, part, &stats);
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

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No puzzle {puzzle}")]
    NoSuchPuzzle { puzzle: u32 },
    #[error(transparent)]
    Report(csv::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<csv::Error> for AocError {
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
    mut visitor: impl FnMut(u32, u32, RuntimeStats, String) -> Result<(), AocError>,
) -> Result<(), AocError> {
    for puzzle in puzzles[1..].iter() {
        puzzle.run(parts, &mut visitor)?;
    }

    Ok(())
}

fn run_one(
    puzzle: u32,
    puzzles: &[Puzzle],
    parts: [bool; 2],
    visitor: impl FnMut(u32, u32, RuntimeStats, String) -> Result<(), AocError>,
) -> Result<(), AocError> {
    let puzzle = puzzles
        .get(puzzle as usize)
        .ok_or(AocError::NoSuchPuzzle { puzzle })?;

    puzzle.run(parts, visitor)
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}