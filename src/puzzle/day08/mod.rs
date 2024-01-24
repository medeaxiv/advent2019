use itertools::Itertools;

use crate::util::{
    bitmap::{Bitmap, BoxDisplay},
    display::OnNewLine,
    position::Position,
};

use super::{Error, Result};

pub const INPUT_FILE: &str = "inputs/day08/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input, 25, 6)
}

fn solve_part1(input: &str, width: u64, height: u64) -> Result<i64> {
    let (_, ones, twos) = input
        .trim()
        .bytes()
        .chunks((width * height) as usize)
        .into_iter()
        .map(|chunk| {
            chunk.fold((0, 0, 0), |(zeros, ones, twos), c| match c {
                b'0' => (zeros + 1, ones, twos),
                b'1' => (zeros, ones + 1, twos),
                b'2' => (zeros, ones, twos + 1),
                _ => (zeros, ones, twos),
            })
        })
        .min_by_key(|&(zeros, ..)| zeros)
        .ok_or_else(|| Error::input("empty input"))?;

    Ok(ones * twos)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    let result = solve_part2(input, 25, 6)?;
    Ok(OnNewLine(BoxDisplay(result)))
}

fn solve_part2(input: &str, width: u64, height: u64) -> Result<Bitmap> {
    let input = input.trim().as_bytes();
    let mut bitmap = Bitmap::new(width, height);

    let layer_stride = (width * height) as usize;
    for (y, x) in (0..height).cartesian_product(0..width) {
        let position = Position::new(x as i64, y as i64);
        let idx = (y * width + x) as usize;

        for idx in (idx..input.len()).step_by(layer_stride) {
            match input[idx] {
                b'0' => {
                    bitmap.put(&position, false);
                    break;
                }
                b'1' => {
                    bitmap.put(&position, true);
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(bitmap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day08/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, "0110")]
    fn test_part2(#[case] which: usize, #[case] expected: &str) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part2(&input, 2, 2)?;
        let expected = solve_part2(expected, 2, 2)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
