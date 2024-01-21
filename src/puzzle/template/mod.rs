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

    #[fixture]
    fn input() -> String {
        std::fs::read_to_string("inputs/template/test.0.txt").expect("Missing test input")
    }

    #[rstest]
    fn test_part1(input: String) {
        let result = solve_part1(&input);
        assert_eq!(result, "TODO");
    }

    #[rstest]
    fn test_part2(input: String) {
        let result = solve_part2(&input);
        assert_eq!(result, "TODO");
    }
}
