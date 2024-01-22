use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use nom_supreme::final_parser::final_parser;

use crate::util::position::{Direction, Movement};

pub fn parse(input: &str) -> Result<Vec<Movement>, nom::error::Error<&str>> {
    final_parser(parser)(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Movement>> {
    separated_list1(tag(","), entry_parser)(input)
}

fn entry_parser(input: &str) -> IResult<&str, Movement> {
    map(
        pair(direction_parser, distance_parser),
        |(direction, distance)| direction * distance,
    )(input)
}

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    map(one_of("UDLR"), |c| match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    })(input)
}

fn distance_parser(input: &str) -> IResult<&str, i64> {
    complete::i64(input)
}
