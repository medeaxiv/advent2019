use super::{Error, Result};

pub const INPUT_FILE: &str = "inputs/day01/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let sum = parse(input)?.into_iter().map(|mass| (mass / 3) - 2).sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let sum = parse(input)?
        .into_iter()
        .flat_map(|mut mass| {
            std::iter::from_fn(move || {
                mass = mass / 3 - 2;
                if mass > 0 {
                    Some(mass)
                } else {
                    None
                }
            })
        })
        .sum();
    Ok(sum)
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|line| line.parse::<i64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("12", 2)]
    #[case("14", 2)]
    #[case("1969", 654)]
    #[case("100756", 33583)]
    fn test_part1(#[case] input: &str, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let result = solve_part1(input)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case("12", 2)]
    #[case("14", 2)]
    #[case("1969", 966)]
    #[case("100756", 50346)]
    fn test_part2(#[case] input: &str, #[case] expected: i64) -> Result<()> {
        crate::util::test::setup_tracing();
        let result = solve_part2(input)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
