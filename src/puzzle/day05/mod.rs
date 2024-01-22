pub const INPUT_FILE: &str = "inputs/template/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(_input: &str) -> &'static str {
    "TODO"
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(_input: &str) -> &'static str {
    "TODO"
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> String {
        let file = format!("inputs/template/test.{}.txt", which);
        std::fs::read_to_string(file).expect("Missing test input file")
    }

    #[rstest]
    #[case(0, "TODO")]
    fn test_part1(#[case] which: usize, #[case] expected: &str) {
        let input = input(which);
        let result = solve_part1(&input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0, "TODO")]
    fn test_part2(#[case] which: usize, #[case] expected: &str) {
        let input = input(which);
        let result = solve_part2(&input);
        assert_eq!(result, expected);
    }
}
