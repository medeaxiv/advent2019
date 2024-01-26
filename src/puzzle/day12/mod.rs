use itertools::Itertools;
use num::integer::lcm;

use crate::util::{
    slice::SliceExt,
    vector::{vec3, Vec3},
};

use super::{Error, Result};

pub const INPUT_FILE: &str = "inputs/day12/input.txt";

mod parser;

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input, 1000)
}

fn solve_part1(input: &str, steps: u64) -> Result<u64> {
    let mut moons = parser::parse(input)?
        .into_iter()
        .map(Body::new)
        .collect::<Box<[_]>>();

    for _ in 0..steps {
        step_system(&mut moons);
    }

    let energy = moons.iter().map(Body::energy).sum();
    Ok(energy)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<u64> {
    let initial_moons = parser::parse(input)?.into_boxed_slice();
    let mut moons = initial_moons
        .iter()
        .copied()
        .map(Body::new)
        .collect::<Box<[_]>>();

    if moons.is_empty() {
        return Err(Error::input("no bodies to simulate"));
    }

    let mut cycles = [0; 3];
    let mut step = 0;
    while cycles.iter().any(|&step| step == 0) {
        step_system(&mut moons);
        step += 1;

        for (coord, cycle) in cycles.iter_mut().enumerate() {
            if *cycle == 0
                && moons.iter().enumerate().all(|(idx, moon)| {
                    moon.position[coord] == initial_moons[idx][coord] && moon.velocity[coord] == 0
                })
            {
                *cycle = step;
            }
        }
    }

    let cycle = cycles
        .into_iter()
        .reduce(lcm)
        .expect("always has 3 elements");
    Ok(cycle)
}

fn step_system(moons: &mut [Body]) {
    for (a_idx, b_idx) in (0..moons.len()).tuple_combinations() {
        let (a, b) = moons
            .multi_index_mut(a_idx, b_idx)
            .expect("a and b index should be different");
        let delta = b.position - a.position;
        let acceleration = vec3(
            delta.x.clamp(-1, 1),
            delta.y.clamp(-1, 1),
            delta.z.clamp(-1, 1),
        );

        a.velocity += acceleration;
        b.velocity -= acceleration;
    }

    for moon in moons.iter_mut() {
        moon.position += moon.velocity;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Body {
    position: Vec3,
    velocity: Vec3,
}

impl Body {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::zeros(),
        }
    }

    pub fn energy(&self) -> u64 {
        let potential = self.position.x.unsigned_abs()
            + self.position.y.unsigned_abs()
            + self.position.z.unsigned_abs();
        let kinetic = self.velocity.x.unsigned_abs()
            + self.velocity.y.unsigned_abs()
            + self.velocity.z.unsigned_abs();
        potential * kinetic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day12/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 10, 179)]
    #[case(1, 100, 1940)]
    fn test_part1(#[case] which: usize, #[case] steps: u64, #[case] expected: u64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part1(&input, steps)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(0, 2772)]
    #[case(1, 4686774924)]
    fn test_part2(#[case] which: usize, #[case] expected: u64) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part2(&input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
