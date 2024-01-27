pub fn fft(signal: &[i64], destination: &mut [i64], len: usize, offset: usize) {
    assert_eq!(signal.len(), destination.len());
    assert!(signal.len() >= len - offset);
    assert!(len >= signal.len());

    let midpoint = len / 2;
    let buffer_delta = len - signal.len();
    let buffer_offset = offset - buffer_delta;
    let buffer_midpoint = if buffer_delta > midpoint {
        0
    } else {
        midpoint - buffer_delta
    };

    if buffer_offset < buffer_midpoint {
        fft_full(
            signal,
            &mut destination[buffer_offset..midpoint],
            len,
            offset,
        );
    }

    let tail_start = buffer_midpoint.max(buffer_offset);
    fft_tail(signal, &mut destination[tail_start..], len, offset);
}

fn fft_full(signal: &[i64], destination: &mut [i64], len: usize, offset: usize) {
    assert_eq!(signal.len(), len);

    for (idx, e) in destination.iter_mut().enumerate() {
        let scale = offset + idx + 1;

        let positive_contributions: i64 = WaveIndex::positive(scale, offset)
            .take_while(|&i| i < signal.len() + offset)
            .map(|i| signal[i - offset])
            .sum();
        let negative_contributions: i64 = WaveIndex::negative(scale, offset)
            .take_while(|&i| i < signal.len() + offset)
            .map(|i| signal[i - offset])
            .sum();

        *e = (positive_contributions - negative_contributions).abs() % 10;
    }
}

fn fft_tail(signal: &[i64], destination: &mut [i64], _len: usize, _offset: usize) {
    let mut accumulator = 0;
    for (idx, e) in destination.iter_mut().rev().enumerate() {
        let signal_idx = signal.len() - idx - 1;
        let next = signal[signal_idx];
        accumulator += next;

        *e = accumulator % 10;
    }
}

struct WaveIndex {
    index: usize,
    phase_end: usize,
    scale: usize,
}

impl WaveIndex {
    const PHASES: usize = 4;

    fn at_phase(phase: usize, scale: usize, offset: usize) -> Self {
        assert!(phase < Self::PHASES);
        if offset == 0 {
            Self {
                index: phase * scale - 1,
                phase_end: (phase + 1) * scale - 1,
                scale,
            }
        } else {
            let global_phase = (offset + 1) / scale;
            let local_phase = global_phase % Self::PHASES;

            let (index, phase_end) = if local_phase != phase {
                let phase_adjustment = phase + Self::PHASES - local_phase;
                let index = (global_phase + phase_adjustment) * scale - 1;
                let phase_end = (global_phase + phase_adjustment + 1) * scale - 1;
                (index, phase_end)
            } else {
                let index = offset;
                let phase_end = (global_phase + 1) * scale - 1;
                (index, phase_end)
            };

            Self {
                index,
                phase_end,
                scale,
            }
        }
    }

    pub fn positive(scale: usize, offset: usize) -> Self {
        Self::at_phase(1, scale, offset)
    }

    pub fn negative(scale: usize, offset: usize) -> Self {
        Self::at_phase(3, scale, offset)
    }
}

impl Iterator for WaveIndex {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.index;

        self.index += 1;
        if self.index >= self.phase_end {
            self.index += 3 * self.scale;
            self.phase_end += 4 * self.scale;
        }

        Some(next)
    }
}

struct Wave {
    index: usize,
    scale: usize,
}

impl Wave {
    const PATTERN: [i64; 4] = [0, 1, 0, -1];

    fn new(scale: usize) -> Self {
        #![allow(dead_code)]
        Self { index: 0, scale }
    }

    fn at(n: usize, scale: usize) -> i64 {
        let idx = (n + 1) / scale % Self::PATTERN.len();
        Self::PATTERN[idx]
    }
}

impl Iterator for Wave {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Self::at(self.index, self.scale);
        self.index += 1;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rstest::*;

    #[rstest]
    #[case(1, [1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0])]
    #[case(2, [0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1])]
    #[case(3, [0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1])]
    fn test_wave(#[case] scale: usize, #[case] expected: impl AsRef<[i64]>) {
        crate::util::test::setup_tracing();
        let expected = expected.as_ref();
        let result = Wave::new(scale).take(expected.len()).collect_vec();
        assert_eq!(&result, expected);
    }

    #[rstest]
    #[case(1, 100)]
    #[case(2, 100)]
    #[case(3, 100)]
    fn test_wave_index(#[case] scale: usize, #[case] length: usize) {
        crate::util::test::setup_tracing();
        let wave = Wave::new(scale).take(length).collect_vec();

        let positive = WaveIndex::positive(scale, 0)
            .take_while(|&idx| idx < length)
            .map(|idx| wave[idx])
            .collect_vec();
        assert_eq!(positive, vec![1; positive.len()]);

        let negative = WaveIndex::negative(scale, 0)
            .take_while(|&idx| idx < length)
            .map(|idx| wave[idx])
            .collect_vec();
        assert_eq!(negative, vec![-1; negative.len()]);
    }
}
