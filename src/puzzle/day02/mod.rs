use itertools::Itertools;

use super::intcode::Intcode;

pub const INPUT_FILE: &str = "inputs/day02/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    let mut program = parse(input);
    program[1] = 12;
    program[2] = 2;
    run(&program)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    let original_program = parse(input);

    let (a, b) = (0..=99)
        .cartesian_product(0..=99)
        .find(|&(a, b)| {
            let mut program = original_program.clone();
            program[1] = a;
            program[2] = b;
            let result = run(&program);
            result == 19690720
        })
        .expect("Values not found");

    100 * a + b
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

fn run(program: &[i64]) -> i64 {
    let mut machine = Intcode::new(program);
    let result = machine.run();

    match result {
        Ok(_) => machine.get_program()[0],
        Err(err) => {
            tracing::error!(error = %err, "Error while executing intcode program");
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> String {
        let file = format!("inputs/day02/test.{}.txt", which);
        std::fs::read_to_string(file).expect("Missing test input file")
    }

    #[rstest]
    #[case(0, 3500)]
    fn test_part1(#[case] which: usize, #[case] expected: i64) {
        crate::util::test::setup_tracing();
        let input = input(which);
        let program = parse(&input);
        let result = run(&program);
        assert_eq!(result, expected);
    }
}
