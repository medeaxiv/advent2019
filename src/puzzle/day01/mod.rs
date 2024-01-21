pub const INPUT_FILE: &str = "inputs/day01/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    input
        .lines()
        .map(|s| s.parse::<i64>().expect("Input should be a valid integer"))
        .map(|mass| (mass / 3) - 2)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .map(|s| s.parse::<i64>().expect("Input should be a valid integer"))
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
        .sum()
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
    fn test_part1(#[case] input: &str, #[case] expected: i64) {
        let result = solve_part1(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("12", 2)]
    #[case("14", 2)]
    #[case("1969", 966)]
    #[case("100756", 50346)]
    fn test_part2(#[case] input: &str, #[case] expected: i64) {
        let result = solve_part2(input);
        assert_eq!(result, expected);
    }
}
