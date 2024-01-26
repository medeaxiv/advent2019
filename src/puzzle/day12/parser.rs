use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    multi::many1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use nom_supreme::final_parser::final_parser;

use crate::util::vector::{vec3, Vec3};

pub fn parse(input: &str) -> Result<Vec<Vec3>, nom::error::Error<&str>> {
    final_parser(parser)(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Vec3>> {
    many1(terminated(moon_parser, line_ending))(input)
}

fn moon_parser(input: &str) -> IResult<&str, Vec3> {
    delimited(tag("<"), position_parser, tag(">"))(input)
}

fn position_parser(input: &str) -> IResult<&str, Vec3> {
    let (input, x) = coordinate_parser("x")(input)?;
    let (input, _) = pair(tag(", "), space0)(input)?;
    let (input, y) = coordinate_parser("y")(input)?;
    let (input, _) = pair(tag(", "), space0)(input)?;
    let (input, z) = coordinate_parser("z")(input)?;

    Ok((input, vec3(x, y, z)))
}

fn coordinate_parser<'a, E>(coordinate: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, i64, E>
where
    E: nom::error::ParseError<&'a str>,
{
    move |input: &'a str| {
        preceded(
            tuple((tag(coordinate), space0, tag("="), space0)),
            complete::i64,
        )(input)
    }
}
