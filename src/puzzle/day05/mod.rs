use itertools::Itertools;

use super::intcode::Intcode;

pub const INPUT_FILE: &str = "inputs/day05/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    let program = parse(input);

    let outputs = match Intcode::run_program_with_inputs(program, [1]) {
        Ok(outputs) => outputs,
        Err(err) => {
            tracing::error!(error = %err, "Error while executing intcode program");
            vec![0]
        }
    };

    outputs[outputs.len() - 1]
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    let program = parse(input);

    let outputs = match Intcode::run_program_with_inputs(program, [5]) {
        Ok(outputs) => outputs,
        Err(err) => {
            tracing::error!(error = %err, "Error while executing intcode program");
            vec![0]
        }
    };

    outputs[outputs.len() - 1]
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<i64>()
                .expect("Input should be a valid integer")
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> String {
        let file = format!("inputs/day05/test.{}.txt", which);
        std::fs::read_to_string(file).expect("Missing test input file")
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(0, 5, 1)]
    #[case(1, 0, 0)]
    #[case(1, 5, 1)]
    #[case(2, 1, 999)]
    #[case(2, 8, 1000)]
    #[case(2, 50, 1001)]
    fn test_part2(#[case] which: usize, #[case] program_input: i64, #[case] expected: i64) {
        crate::util::test::setup_tracing();
        let input = input(which);
        let program = parse(&input);

        let result = match Intcode::run_program_with_inputs(program, [program_input]) {
            Ok(outputs) => outputs,
            Err(err) => {
                tracing::error!(error = %err, "Error while executing intcode program");
                panic!("Error while executing intcode program");
            }
        };

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
    }
}
