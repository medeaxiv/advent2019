use itertools::Itertools;
use num::Zero;

mod parser;

use crate::util::{
    geometry::manhattan_distance,
    numerics::min_max,
    position::{Orientation, Position},
};

pub const INPUT_FILE: &str = "inputs/day03/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    let (a, b) = input
        .lines()
        .map(path)
        .collect_tuple()
        .expect("Input should have 2 paths");

    let a_iter = a
        .iter()
        .map(|(p, _)| *p)
        .tuple_windows::<(Position, Position)>();
    let b_iter = b
        .iter()
        .map(|(p, _)| *p)
        .tuple_windows::<(Position, Position)>();

    a_iter
        .cartesian_product(b_iter)
        .filter_map(|(a, b)| segment_intersection(&a, &b))
        .filter(|intersection| !intersection.is_zero())
        .map(|intersection| manhattan_distance(Position::zeros(), intersection))
        .reduce(i64::min)
        .expect("No intersection found")
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    let (a, b) = input
        .lines()
        .map(path)
        .collect_tuple()
        .expect("Input should have 2 paths");

    let a_iter = a.iter().tuple_windows().map(|(a, b)| ((a.0, b.0), a.1));
    let b_iter = b.iter().tuple_windows().map(|(a, b)| ((a.0, b.0), a.1));

    a_iter
        .cartesian_product(b_iter)
        .filter_map(|((a, a_dist), (b, b_dist))| {
            segment_intersection(&a, &b)
                .filter(|intersection| !intersection.is_zero())
                .map(|intersection| {
                    let a_dist = a_dist + manhattan_distance(a.0, intersection);
                    let b_dist = b_dist + manhattan_distance(b.0, intersection);
                    a_dist + b_dist
                })
        })
        .reduce(i64::min)
        .expect("No intersection found")
}

fn segment_intersection(a: &(Position, Position), b: &(Position, Position)) -> Option<Position> {
    let a_orientation = if a.0.x == a.1.x {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    };

    let b_orientation = if b.0.x == b.1.x {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    };

    if a_orientation == b_orientation {
        return None;
    }

    match a_orientation {
        Orientation::Horizontal => orthogonal_intersection(a, b),
        Orientation::Vertical => orthogonal_intersection(b, a),
    }
}

fn orthogonal_intersection(
    horizontal: &(Position, Position),
    vertical: &(Position, Position),
) -> Option<Position> {
    let p = Position::new(vertical.0.x, horizontal.0.y);

    let (min_x, max_x) = min_max(horizontal.0.x, horizontal.1.x);
    let (min_y, max_y) = min_max(vertical.0.y, vertical.1.y);
    if (min_x..=max_x).contains(&p.x) && (min_y..=max_y).contains(&p.y) {
        Some(p)
    } else {
        None
    }
}

fn path(input: &str) -> Vec<(Position, i64)> {
    let instructions = parser::parse(input).expect("Invalid path declaration");
    let mut path = Vec::with_capacity(instructions.len() + 1);

    let mut position = Position::zeros();
    let mut distance = 0;
    path.push((position, distance));
    for movement in instructions {
        position += movement;
        distance += movement.distance as i64;
        path.push((position, distance));
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> String {
        let file = format!("inputs/day03/test.{}.txt", which);
        std::fs::read_to_string(file).expect("Missing test input file")
    }

    #[rstest]
    #[case(0, 159)]
    #[case(1, 135)]
    #[case(2, 6)]
    fn test_part1(#[case] which: usize, #[case] expected: i64) {
        crate::util::test::setup_tracing();
        let input = input(which);
        let result = solve_part1(&input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0, 610)]
    #[case(1, 410)]
    #[case(2, 30)]
    fn test_part2(#[case] which: usize, #[case] expected: i64) {
        crate::util::test::setup_tracing();
        let input = input(which);
        let result = solve_part2(&input);
        assert_eq!(result, expected);
    }
}
