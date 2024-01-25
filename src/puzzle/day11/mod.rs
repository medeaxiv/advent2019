use ahash::AHashMap as HashMap;
use itertools::Itertools;
use nalgebra::SimdPartialOrd;

use crate::util::{
    bitmap::{Bitmap, BoxDisplay},
    display::OnNewLine,
    position::{Direction, Position},
};

use super::{
    intcode::{self, Intcode, State},
    Error, Result,
};

pub const INPUT_FILE: &str = "inputs/day11/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<usize> {
    let program = intcode::parse_program(input)?;
    let panels = run_robot(program, 0)?;
    Ok(panels.len())
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input).map(|bitmap| OnNewLine(BoxDisplay(bitmap)))
}

fn solve_part2(input: &str) -> Result<Bitmap> {
    let program = intcode::parse_program(input)?;
    let panels = run_robot(program, 1)?;

    let (min, max) = {
        let mut white_panels =
            panels
                .iter()
                .filter_map(|(&position, &color)| if color != 0 { Some(position) } else { None });
        let Some(first) = white_panels.next() else {
            return Ok(Bitmap::new(0, 0));
        };

        white_panels.fold((first, first), |(min, max), next| {
            (min.simd_min(next), max.simd_max(next))
        })
    };

    let size = max - min + Position::new(1, 1);
    let mut bitmap = Bitmap::new(size.x.unsigned_abs(), size.y.unsigned_abs());

    for (y, x) in (min.y..=max.y).cartesian_product(min.x..=max.x) {
        let position = Position::new(x, y);
        let color = panels.get(&position).copied().unwrap_or(0);
        let position = position - min;
        let value = color != 0;
        bitmap.put(&position, value)
    }

    Ok(bitmap)
}

fn run_robot(program: impl AsRef<[i64]>, starting_color: i64) -> Result<HashMap<Position, i64>> {
    let mut machine = Intcode::new(program);

    let mut position = Position::zeros();
    let mut direction = Direction::Up;
    let mut panels = HashMap::from([(position, starting_color)]);

    loop {
        // Run program
        let color = panels.get(&position).copied().unwrap_or(0);
        machine.push_input(color);
        machine.run()?;
        let color = machine.pop_output().ok_or(intcode::Error::MissingOutput)?;
        let turn = machine.pop_output().ok_or(intcode::Error::MissingOutput)?;

        // Paint panel
        panels.insert(position, color);

        // Move robot
        direction = match turn {
            0 => direction.turn_left(),
            1 => direction.turn_right(),
            _ => return Err(Error::execution(&format!("unexpected turn {turn}"))),
        };
        position += direction;

        if machine.get_state() == State::Terminated {
            break;
        }
    }

    Ok(panels)
}
