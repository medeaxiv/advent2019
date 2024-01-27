use ahash::AHashMap as HashMap;

use crate::util::position::{Direction, Position};

use super::{
    intcode::{self, Intcode},
    Error, Result,
};

pub const INPUT_FILE: &str = "inputs/day15/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<usize> {
    let program = intcode::parse_program(input)?;
    let path_length = find_path_length_to_oxygen_supply(program)?;
    Ok(path_length)
}

fn find_path_length_to_oxygen_supply(program: impl AsRef<[i64]>) -> Result<usize> {
    fn explore(
        from: Position,
        machine: &mut Intcode,
        path: &mut Vec<Direction>,
        map: &mut HashMap<Position, Tile>,
    ) -> Result<Option<usize>> {
        for direction in Direction::ALL {
            let position = from + direction;
            if map.contains_key(&position) {
                continue;
            }

            machine.push_input(direction_input(direction));
            machine.run()?;
            let out = machine.pop_output().ok_or(intcode::Error::MissingOutput)?;
            match out {
                0 => {
                    map.insert(position, Tile::Wall);
                    continue;
                }
                1 => {
                    map.insert(position, Tile::Empty);
                }
                2 => return Ok(Some(path.len() + 1)),
                _ => return Err(Error::execution(&format!("unexpected output {out}"))),
            };

            path.push(direction);
            if let Some(path_length) = explore(position, machine, path, map)? {
                return Ok(Some(path_length));
            }
            path.pop();

            machine.push_input(direction_input(direction.inverse()));
            machine.run()?;
            machine.pop_output();
        }

        Ok(None)
    }

    let position = Position::zeros();
    let mut machine = Intcode::new(program);
    let mut path = Vec::new();
    let mut map = HashMap::from([(position, Tile::Empty)]);

    let path_length = explore(position, &mut machine, &mut path, &mut map)?;
    path_length.ok_or_else(|| Error::search("no oxygen supply found"))
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<usize> {
    let program = intcode::parse_program(input)?;
    let (map, oxygen_supply) = explore_map(program)?;

    let mut max_depth = 0;
    crate::util::graph::search::breadth_first_search(
        |&position, _| {
            let neighbors = Direction::ALL.map(|direction| position + direction);
            neighbors.into_iter().filter(|neighbor| {
                map.get(neighbor)
                    .is_some_and(|tile| matches!(tile, Tile::Empty))
            })
        },
        |_, depth| {
            max_depth = max_depth.max(depth);
            None as Option<()>
        },
        [oxygen_supply],
    );

    Ok(max_depth)
}

fn explore_map(program: impl AsRef<[i64]>) -> Result<(HashMap<Position, Tile>, Position)> {
    fn explore(
        from: Position,
        machine: &mut Intcode,
        path: &mut Vec<Direction>,
        map: &mut HashMap<Position, Tile>,
    ) -> Result<Option<Position>> {
        let mut destination = None;

        for direction in Direction::ALL {
            let position = from + direction;
            if map.contains_key(&position) {
                continue;
            }

            machine.push_input(direction_input(direction));
            machine.run()?;
            let out = machine.pop_output().ok_or(intcode::Error::MissingOutput)?;
            match out {
                0 => {
                    map.insert(position, Tile::Wall);
                    continue;
                }
                1 => {
                    map.insert(position, Tile::Empty);
                }
                2 => {
                    destination = Some(position);
                    map.insert(position, Tile::Empty);
                }
                _ => return Err(Error::execution(&format!("unexpected output {out}"))),
            };

            path.push(direction);
            let maybe_destination = explore(position, machine, path, map)?;
            path.pop();

            if destination.is_none() {
                destination = maybe_destination;
            }

            machine.push_input(direction_input(direction.inverse()));
            machine.run()?;
            machine.pop_output();
        }

        Ok(destination)
    }

    let position = Position::zeros();
    let mut machine = Intcode::new(program);
    let mut path = Vec::new();
    let mut map = HashMap::from([(position, Tile::Empty)]);

    let destination = explore(position, &mut machine, &mut path, &mut map)?;
    destination
        .map(|destination| (map, destination))
        .ok_or_else(|| Error::search("no oxygen supply found"))
}

fn direction_input(direction: Direction) -> i64 {
    match direction {
        Direction::Up => 1,
        Direction::Down => 2,
        Direction::Left => 3,
        Direction::Right => 4,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}
