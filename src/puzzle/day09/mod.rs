use super::{
    intcode::{self, Intcode},
    Result,
};

pub const INPUT_FILE: &str = "inputs/day09/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let program = intcode::parse_program(input)?;
    let outputs = Intcode::run_program_with_inputs(program, [1])?;
    Ok(outputs[outputs.len() - 1])
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let program = intcode::parse_program(input)?;
    let outputs = Intcode::run_program_with_inputs(program, [2])?;
    Ok(outputs[outputs.len() - 1])
}
