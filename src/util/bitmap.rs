use std::fmt::Write;

use super::position::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    width: i64,
    height: i64,
    bits: Vec<u64>,
}

impl Bitmap {
    pub fn new(width: u64, height: u64) -> Self {
        let expand = (width * height) % 64 != 0;
        let len = (width * height) / 64 + (if expand { 1 } else { 0 });
        let contents = vec![0; len as usize];

        Self {
            width: width as i64,
            height: height as i64,
            bits: contents,
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn contains(&self, position: &Position) -> bool {
        (0..self.width).contains(&position.x) && (0..self.height).contains(&position.y)
    }

    fn index(&self, position: &Position) -> Option<(usize, usize)> {
        if self.contains(position) {
            let idx = (position.y * self.width + position.x) as usize;
            Some((idx / 64, idx % 64))
        } else {
            None
        }
    }

    pub fn get(&self, position: &Position) -> bool {
        if let Some((idx, bit)) = self.index(position) {
            let mask = 1 << bit;
            let value = self.bits[idx] & mask;
            value != 0
        } else {
            false
        }
    }

    pub fn get_bit(&self, position: &Position) -> u8 {
        if self.get(position) {
            1
        } else {
            0
        }
    }

    pub fn put(&mut self, position: &Position, value: bool) {
        if let Some((idx, bit)) = self.index(position) {
            let mask = 1 << bit;
            let inv_mask = u64::MAX ^ mask;
            let value = if value { u64::MAX } else { 0 };
            self.bits[idx] = (self.bits[idx] & inv_mask) | (value & mask);
        }
    }
}

pub trait BitmapDisplay: std::fmt::Display {}

pub struct BoxDisplay(pub Bitmap);

impl BoxDisplay {
    fn get_char(&self, x: i64, y: i64) -> char {
        let upper = self.0.get(&Position::new(x, y * 2));
        let lower = self.0.get(&Position::new(x, y * 2 + 1));
        match (upper, lower) {
            (true, true) => '\u{2588}',
            (true, false) => '\u{2580}',
            (false, true) => '\u{2584}',
            (false, false) => ' ',
        }
    }
}

impl BitmapDisplay for BoxDisplay {}
impl std::fmt::Display for BoxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.0.width();
        let height = (self.0.height() + self.0.height() % 2) / 2;

        for y in 0..height {
            for x in 0..width {
                f.write_char(self.get_char(x, y))?;
            }

            if y + 1 < height {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

pub struct BrailleDisplay(pub Bitmap);

impl BrailleDisplay {
    fn get_char(&self, x: i64, y: i64) -> char {
        let pattern = self.0.get_bit(&Position::new(x * 2, y * 4))
            | self.0.get_bit(&Position::new(x * 2, y * 4 + 1)) << 1
            | self.0.get_bit(&Position::new(x * 2, y * 4 + 2)) << 2
            | self.0.get_bit(&Position::new(x * 2 + 1, y * 4)) << 3
            | self.0.get_bit(&Position::new(x * 2 + 1, y * 4 + 1)) << 4
            | self.0.get_bit(&Position::new(x * 2 + 1, y * 4 + 2)) << 5
            | self.0.get_bit(&Position::new(x * 2, y * 4 + 3)) << 6
            | self.0.get_bit(&Position::new(x * 2 + 1, y * 4 + 3)) << 7;
        let codepoint = 0x2800 | pattern as u32;

        unsafe { char::from_u32_unchecked(codepoint) }
    }
}

impl BitmapDisplay for BrailleDisplay {}
impl std::fmt::Display for BrailleDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = (self.0.width() + self.0.width() % 2) / 2;
        let height = (self.0.height() + self.0.height() % 4) / 4;

        for y in 0..height {
            for x in 0..width {
                f.write_char(self.get_char(x, y))?;
            }

            if y + 1 < height {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
