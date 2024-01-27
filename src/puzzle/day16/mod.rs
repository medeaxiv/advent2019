use super::{Error, Result};

pub const INPUT_FILE: &str = "inputs/day16/input.txt";

mod fft;

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    let result = solve_part1(input, 100)?;
    let bytes = Vec::from(result.map(|v| v as u8 + b'0'));
    let string = String::from_utf8(bytes).expect("invalid utf8");
    Ok(string)
}

fn solve_part1(input: &str, rounds: usize) -> Result<[i64; 8]> {
    let mut signal = parse(input)?;
    let mut next = signal.clone();

    for _ in 0..rounds {
        fft::fft(&signal, &mut next, signal.len(), 0);
        std::mem::swap(&mut signal, &mut next);
    }

    let mut result = [0; 8];
    result.copy_from_slice(&signal[0..8]);
    Ok(result)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    let result = solve_part2(input, 100)?;
    let bytes = Vec::from(result.map(|v| v as u8 + b'0'));
    let string = String::from_utf8(bytes).expect("invalid utf8");
    Ok(string)
}

fn solve_part2(input: &str, rounds: usize) -> Result<[i64; 8]> {
    let input = parse(input)?;
    let offset = {
        let mut rank = 1;
        let mut offset = 0;
        for digit in input[0..7].iter().rev() {
            offset += rank * digit;
            rank *= 10;
        }

        offset as usize
    };

    let signal_len = input.len() * 10_000;
    let buffer_len = signal_len - offset;

    assert!(offset > signal_len / 2);

    let mut signal = input
        .repeat(buffer_len / input.len() + 1)
        .into_boxed_slice();
    let mut next = signal.clone();

    for _ in 0..rounds {
        fft::fft(&signal, &mut next, signal_len, offset);
        std::mem::swap(&mut signal, &mut next);
    }

    let signal_offset = signal.len() - buffer_len;
    let mut result = [0; 8];
    result.copy_from_slice(&signal[signal_offset..signal_offset + 8]);
    Ok(result)
}

fn parse(input: &str) -> Result<Box<[i64]>> {
    input
        .trim()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_digit() {
                let value = byte - b'0';
                Ok(value as i64)
            } else {
                Err(Error::parse("not a digit"))
            }
        })
        .collect::<Result<Box<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn input(which: usize) -> Result<String> {
        let file = format!("inputs/day16/test.{}.txt", which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(0, 4,   [0,1,0,2,9,4,9,8])]
    #[case(1, 100, [2,4,1,7,6,1,7,6])]
    #[case(2, 100, [7,3,7,4,5,4,1,8])]
    #[case(3, 100, [5,2,4,3,2,1,3,3])]
    fn test_part1(
        #[case] which: usize,
        #[case] rounds: usize,
        #[case] expected: [i64; 8],
    ) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part1(&input, rounds)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(4, 100, [8,4,4,6,2,0,2,6])]
    #[case(5, 100, [7,8,7,2,5,2,7,0])]
    #[case(6, 100, [5,3,5,5,3,7,3,1])]
    fn test_part2(
        #[case] which: usize,
        #[case] rounds: usize,
        #[case] expected: [i64; 8],
    ) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(which)?;
        let result = solve_part2(&input, rounds)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
