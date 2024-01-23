use itertools::Itertools;

use super::{
    intcode::{self, Intcode},
    Error, Result,
};

pub const INPUT_FILE: &str = "inputs/day07/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let program = intcode::parse_program(input)?;

    let result = (0..=4)
        .permutations(5)
        .flat_map(|phase| amplify_once(0, &phase, &program))
        .max()
        .ok_or_else(|| Error::search("no working phase settings"))?;

    Ok(result)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let program = intcode::parse_program(input)?;

    let result = (5..=9)
        .permutations(5)
        .flat_map(|phase| amplify(0, &phase, &program))
        .max()
        .ok_or_else(|| Error::search("no working phase settings"))?;

    Ok(result)
}

fn amplify_once(mut signal: i64, phase: &[i64], program: &[i64]) -> intcode::Result<i64> {
    for &phase in phase.iter() {
        let mut machine = Intcode::new(program);
        machine.push_input(phase);
        machine.push_input(signal);
        machine.run()?;
        signal = machine.pop_output().ok_or(intcode::Error::MissingOutput)?;
    }

    Ok(signal)
}

fn amplify(mut signal: i64, phase: &[i64], program: &[i64]) -> intcode::Result<i64> {
    let mut machines = phase
        .iter()
        .map(|&phase| {
            let mut machine = Intcode::new(program);
            machine.push_input(phase);
            machine
        })
        .collect_vec();
    let last_idx = machines.len() - 1;

    loop {
        for machine in machines.iter_mut() {
            machine.push_input(signal);
            machine.run()?;
            signal = machine.pop_output().ok_or(intcode::Error::MissingOutput)?;
        }

        if machines[last_idx].get_state() == intcode::State::Terminated {
            break;
        }
    }

    Ok(signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day07/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 43210)]
    #[case(1, 54321)]
    #[case(2, 65210)]
    fn test_part1(#[case] which: usize, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part1(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(3, 139629729)]
    #[case(4, 18216)]
    fn test_part2(#[case] which: usize, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part2(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
