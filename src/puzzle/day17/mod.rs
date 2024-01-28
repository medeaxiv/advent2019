use ahash::AHashSet as HashSet;
use itertools::Itertools;

use crate::util::position::{pos, Direction, Position};

use super::{
    intcode::{self, Intcode},
    Error, Result,
};

pub const INPUT_FILE: &str = "inputs/day17/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let program = intcode::parse_program(input)?;
    let output = Intcode::run_program_with_inputs(program, [])?;
    let screen = Screen::try_from(&output[..])?;
    let result = calibration(&screen);
    Ok(result)
}

fn calibration(screen: &Screen) -> i64 {
    screen
        .scaffolding
        .iter()
        .copied()
        .filter(|&position| {
            Direction::ALL
                .into_iter()
                .all(|direction| screen.scaffolding.contains(&(position + direction)))
        })
        .map(|position| position.x * position.y)
        .sum()
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    // Solved for my input with a "use eyes" algorithm
    let main_routine = "A,A,C,B,C,B,C,B,A,B\n";
    let a_program = "R,6,L,8,R,8\n";
    let b_program = "L,8,R,6,L,10,L,10\n";
    let c_program = "R,4,R,6,R,6,R,4,R,4\n";

    let mut program = intcode::parse_program(input)?;
    program[0] = 2;
    let mut machine = Intcode::new(program);
    machine.push_text_input(main_routine);
    machine.push_text_input(a_program);
    machine.push_text_input(b_program);
    machine.push_text_input(c_program);
    machine.push_text_input("n\n");
    machine.run()?;
    let output = machine.drain_output();

    Ok(output[output.len() - 1])
}

fn printout(machine: &mut Intcode) {
    #![allow(dead_code)]
    let output = machine.drain_output();
    let bytes = output.into_iter().map(|n| n as u8).collect_vec();
    println!("{}", String::from_utf8(bytes).expect("utf8"));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position: Position,
    direction: Direction,
    is_dead: bool,
}

struct Screen {
    robot: Robot,
    scaffolding: HashSet<Position>,
}

impl TryFrom<&[i64]> for Screen {
    type Error = Error;

    fn try_from(value: &[i64]) -> Result<Self> {
        let mut robot = Robot::default();
        let mut scaffolding = HashSet::new();

        let (mut x, mut y) = (0, 0);
        let mut previous = None;
        for value in value.iter().copied() {
            if let Some(previous) = previous {
                if previous == 10 && value == 10 {
                    break;
                }
            }

            previous = Some(value);
            let byte = value as u8;
            if byte == 10 {
                y += 1;
                x = 0;
                continue;
            }

            if matches!(byte, b'#' | b'^' | b'v' | b'<' | b'>') {
                scaffolding.insert(pos(x, y));
            }

            match byte {
                b'X' => {
                    robot.position = pos(x, y);
                    robot.is_dead = true;
                }
                b'^' | b'v' | b'<' | b'>' => {
                    robot.position = pos(x, y);
                    robot.direction = dir(byte)?;
                    robot.is_dead = false;
                }
                _ => {}
            }

            x += 1;
        }

        Ok(Self { robot, scaffolding })
    }
}

impl TryFrom<&str> for Screen {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let value = value
            .bytes()
            .filter(|&byte| byte != b'\r')
            .map(|byte| byte as i64)
            .collect::<Box<_>>();
        println!("{value:?}");
        Self::try_from(value.as_ref())
    }
}

fn dir(value: u8) -> Result<Direction> {
    match value {
        b'^' => Ok(Direction::Up),
        b'v' => Ok(Direction::Down),
        b'<' => Ok(Direction::Left),
        b'>' => Ok(Direction::Right),
        _ => Err(Error::parse(&format!("unknown direction value {value}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day17/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 76)]
    fn test_part1(#[case] which: usize, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let screen = Screen::try_from(input.as_str())?;
        let result = calibration(&screen);
        assert_eq!(result, expected);
        Ok(())
    }
}
