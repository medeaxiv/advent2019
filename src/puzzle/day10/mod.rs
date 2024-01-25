use ahash::AHashMap as HashMap;
use itertools::Itertools;
use num::{integer::gcd, Rational64};
use rayon::prelude::*;

use crate::util::{geometry::manhattan_distance, position::Position};

use super::{Error, Result};

pub const INPUT_FILE: &str = "inputs/day10/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<usize> {
    let asteroids = parse(input)?;

    let result = asteroids
        .par_iter()
        .map(|&position| count_visible_asteroids(position, &asteroids))
        .max()
        .unwrap_or(0);
    Ok(result)
}

fn count_visible_asteroids(position: Position, asteroids: &[Position]) -> usize {
    asteroids
        .iter()
        .filter(|&&asteroid| asteroid != position)
        .map(|&asteroid| Slope::between(position, asteroid))
        .unique()
        .count()
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let asteroids = parse(input)?;

    let index = asteroids
        .par_iter()
        .map(|&position| index_asteroids_by_slope(position, &asteroids))
        .max_by_key(|index| index.len())
        .ok_or_else(|| Error::input("no asteroids"))?;

    let slopes = index.keys().copied().sorted_unstable().collect_vec();

    let mut cycle = 0;
    let mut counter = 0;
    let mut target = None;
    'search: loop {
        let mut found_targets = false;
        for slope in slopes.iter() {
            let targets = &index[slope];
            if targets.len() > cycle {
                found_targets = true;
                counter += 1;
            }

            if counter == 200 {
                let asteroid = asteroids[targets[cycle].0];
                target = Some(asteroid);
                break 'search;
            }
        }
        cycle += 1;

        if !found_targets {
            break;
        }
    }

    let asteroid = target.ok_or_else(|| Error::search("unable to find 200th asteroid"))?;
    Ok(100 * asteroid.x + asteroid.y)
}

fn index_asteroids_by_slope(
    position: Position,
    asteroids: &[Position],
) -> HashMap<Slope, Vec<(usize, i64)>> {
    let mut index: HashMap<Slope, Vec<(usize, i64)>> = HashMap::new();

    for (idx, &asteroid) in asteroids
        .iter()
        .filter(|&&asteroid| asteroid != position)
        .enumerate()
    {
        let slope = Slope::between(position, asteroid);
        let distance = manhattan_distance(position, asteroid);

        if let Some(entry) = index.get_mut(&slope) {
            // let insert_idx = entry.partition_point(|&(_, d)| d < distance);
            let insert_idx = 0;
            entry.insert(insert_idx, (idx, distance));
        } else {
            index.insert(slope, vec![(idx, distance)]);
        }
    }

    index
}

fn parse(input: &str) -> Result<Vec<Position>> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, byte)| (x as i64, y as i64, byte))
        })
        .filter_map(|(x, y, byte)| match byte {
            b'.' => None,
            _ => Some(Position::new(x, y)),
        })
        .collect();
    Ok(positions)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Slope {
    rise: i64,
    run: i64,
}

impl Slope {
    fn between(a: Position, b: Position) -> Self {
        let delta = b - a;
        let gcd = gcd(delta.x, delta.y);
        Self {
            rise: delta.y / gcd,
            run: delta.x / gcd,
        }
    }
}

impl Ord for Slope {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        if self == other {
            return Equal;
        }

        match (self.run, other.run) {
            (0, 0) => self.rise.cmp(&other.rise),
            (0, b) => {
                if self.rise < 0 || b < 0 {
                    Less
                } else {
                    Greater
                }
            }
            (a, 0) => {
                if other.rise < 0 || a < 0 {
                    Greater
                } else {
                    Less
                }
            }
            (a, b) if a.signum() != b.signum() => {
                // a and b have opposite signs, negative will be greater due to clockwise rotation
                b.cmp(&a)
            }
            (_a, _b) => {
                // a and b have the same sign, compare slopes directly
                let a = Rational64::new_raw(self.rise, self.run);
                let b = Rational64::new_raw(other.rise, other.run);
                a.cmp(&b)
            }
        }
    }
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day10/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 8)]
    #[case(1, 33)]
    #[case(2, 35)]
    #[case(3, 41)]
    #[case(4, 210)]
    fn test_part1(#[case] which: usize, #[case] expected: usize) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part1(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(4, 802)]
    fn test_part2(#[case] which: usize, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part2(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
