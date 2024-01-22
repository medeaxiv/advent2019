use itertools::Itertools;

pub const INPUT_FILE: &str = "inputs/day04/input.txt";

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    let (start, end) = parse(input);

    let start_digits = Digits::<6>::from(start);
    let candidate_iter = std::iter::successors(Some(start_digits), |digits| {
        Some(next_candidate_password(*digits))
    });

    candidate_iter
        .filter(has_any_duplicate_pair)
        .take_while(|password| {
            let value = password.as_integer();
            end >= value
        })
        .count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> usize {
    let (start, end) = parse(input);

    let start_digits = Digits::<6>::from(start);
    let candidate_iter = std::iter::successors(Some(start_digits), |digits| {
        Some(next_candidate_password(*digits))
    });

    candidate_iter
        .filter(has_exact_duplicate_pair)
        .take_while(|password| {
            let value = password.as_integer();
            end >= value
        })
        .count()
}

fn parse(input: &str) -> (u64, u64) {
    let (a, b) = input.trim().split_once('-').expect("Invalid input");

    (
        a.parse().expect("Invalid input"),
        b.parse().expect("Invalid input"),
    )
}

fn next_candidate_password<const S: usize>(mut digits: Digits<S>) -> Digits<S> {
    digits.increment();

    let mut overwriting = false;
    let mut max_digit = 0;
    for idx in 0..S {
        let digit = digits[idx];

        if digit < max_digit {
            overwriting = true;
        }

        if overwriting {
            digits[idx] = max_digit;
        } else {
            max_digit = digits[idx];
        }
    }

    digits
}

fn has_any_duplicate_pair<const S: usize>(digits: &Digits<S>) -> bool {
    digits
        .digits
        .iter()
        .copied()
        .tuple_windows()
        .any(|(a, b)| a == b)
}

fn has_exact_duplicate_pair<const S: usize>(digits: &Digits<S>) -> bool {
    for (_, group) in digits.digits.iter().group_by(|&d| *d).into_iter() {
        if group.count() == 2 {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Digits<const S: usize> {
    digits: [u8; S],
}

impl<const S: usize> Digits<S> {
    pub fn from_integer(mut integer: u64) -> Self {
        let mut digits = [0; S];

        for i in 0..S {
            let idx = S - i - 1;
            digits[idx] = (integer % 10) as u8;
            integer /= 10;
        }

        Self { digits }
    }

    pub fn as_integer(&self) -> u64 {
        let mut value = 0;

        let mut rank = 1;
        for i in 0..S {
            let idx = S - i - 1;
            value += self.digits[idx] as u64 * rank;
            rank *= 10;
        }

        value
    }

    pub fn increment(&mut self) {
        for i in 0..S {
            let idx = S - i - 1;
            if self.digits[idx] < 9 {
                self.digits[idx] += 1;
                break;
            } else {
                self.digits[idx] = 0;
            }
        }
    }
}

impl<const S: usize> From<u64> for Digits<S> {
    fn from(value: u64) -> Self {
        Self::from_integer(value)
    }
}

impl<const S: usize> From<Digits<S>> for u64 {
    fn from(value: Digits<S>) -> Self {
        value.as_integer()
    }
}

impl<const S: usize> std::ops::Index<usize> for Digits<S> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digits[index]
    }
}

impl<const S: usize> std::ops::IndexMut<usize> for Digits<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.digits[index]
    }
}
