use itertools::Itertools;

pub const INPUT_FILE: &str = "inputs/day02/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    let mut program = parse(input);
    program[1] = 12;
    program[2] = 2;
    run(&mut program)
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
            let result = run(&mut program);
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

fn run(program: &mut [i64]) -> i64 {
    let mut ip = 0;
    loop {
        let opcode = program[ip];
        let value = match opcode {
            1 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                a + b
            }
            2 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                a * b
            }
            99 => {
                break;
            }
            _ => panic!("unexpected opcode {opcode:0.2x}"),
        };

        let address = program[ip + 3] as usize;
        tracing::debug!(value, address, "storing value");
        program[address] = value;
        ip += 4;
    }

    program[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> String {
        std::fs::read_to_string("inputs/day02/test.0.txt").expect("Missing test input")
    }

    #[rstest]
    fn test_part1(input: String) {
        crate::util::test::setup_tracing();
        let mut program = parse(&input);
        let result = run(&mut program);
        assert_eq!(result, 3500);
    }
}
