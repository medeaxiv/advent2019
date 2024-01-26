use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use nom_supreme::final_parser::final_parser;

use super::{Reaction, Reactions};

pub fn parse(input: &str) -> Result<Reactions, nom::error::Error<&str>> {
    let list = final_parser(parser)(input)?;

    let mut reactions = Reactions::default();
    reactions.add_chemical("FUEL");
    reactions.add_chemical("ORE");
    for (chemical, reaction) in list {
        reactions.add_reaction(chemical, reaction);
    }

    Ok(reactions)
}

fn parser(input: &str) -> IResult<&str, Vec<(&str, Reaction<&str>)>> {
    many1(terminated(reaction_parser, line_ending))(input)
}

fn reaction_parser(input: &str) -> IResult<&str, (&str, Reaction<&str>)> {
    map(
        separated_pair(ingredients_parser, tag(" => "), component_parser),
        |(ingredients, result)| {
            (
                result.0,
                Reaction {
                    amount_produced: result.1,
                    ingredients,
                },
            )
        },
    )(input)
}

fn ingredients_parser(input: &str) -> IResult<&str, Vec<(&str, u64)>> {
    separated_list1(tag(", "), component_parser)(input)
}

fn component_parser(input: &str) -> IResult<&str, (&str, u64)> {
    map(
        separated_pair(complete::u64, space1, alpha1),
        |(count, chemical)| (chemical, count),
    )(input)
}
