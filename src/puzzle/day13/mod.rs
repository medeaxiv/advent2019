use ahash::AHashMap as HashMap;
use itertools::Itertools;
use nalgebra::SimdPartialOrd;

use crate::util::{
    bitmap::{Bitmap, BoxFormatter},
    display::OnNewLine,
    position::{pos, Position},
    vector::vec2,
};

use super::{
    intcode::{self, Intcode, State},
    Error, Result,
};

pub const INPUT_FILE: &str = "inputs/day13/input.txt";

pub fn part1(input: &str) -> Result<impl std::fmt::Display> {
    solve_part1(input)
}

fn solve_part1(input: &str) -> Result<i64> {
    let program = intcode::parse_program(input)?;
    let mut arcade = Arcade::new(program);
    let _ = arcade.update()?;
    Ok(arcade.blocks)
}

pub fn part2(input: &str) -> Result<impl std::fmt::Display> {
    solve_part2(input)
}

fn solve_part2(input: &str) -> Result<i64> {
    let mut program = intcode::parse_program(input)?;
    program[0] = 2;
    let mut arcade = Arcade::new(program);
    arcade.run()?;
    Ok(arcade.score)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = Error;

    fn try_from(value: i64) -> core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(Error::execution(&format!("no tile of type {value}"))),
        }
    }
}

struct Arcade {
    machine: Intcode,
    screen: HashMap<Position, Tile>,
    bitmap: Option<Bitmap>,
    score: i64,
    blocks: i64,
    paddle: i64,
    ball: i64,
}

impl Arcade {
    pub fn new(program: impl AsRef<[i64]>) -> Self {
        let machine = Intcode::new(program);
        let screen = HashMap::new();

        Self {
            machine,
            screen,
            bitmap: None,
            score: 0,
            blocks: 0,
            paddle: 0,
            ball: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        #![allow(dead_code)]

        loop {
            self.step()?;

            if self.blocks == 0 {
                break;
            }
        }

        Ok(())
    }

    pub fn run_presenting(&mut self) -> Result<()> {
        #![allow(dead_code)]
        use std::time::Duration;

        loop {
            self.step()?;
            self.present();
            std::thread::sleep(Duration::from_millis(15));

            if self.blocks == 0 {
                break;
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        let state = self.update()?;

        if state == State::WaitingForInput {
            let input = (self.ball - self.paddle).signum();
            self.machine.push_input(input);
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<State> {
        self.machine.run()?;

        for (position, value) in std::iter::from_fn(|| self.machine.pop_output())
            .tuples()
            .map(|(x, y, value)| (pos(x, y), value))
        {
            if position.x < 0 {
                self.score = value;
                continue;
            }

            let tile = value.try_into()?;
            match tile {
                Tile::Ball => {
                    self.ball = position.x;
                }
                Tile::Paddle => {
                    self.paddle = position.x;
                }
                Tile::Block => {
                    self.blocks += 1;
                }
                _ => {}
            }

            self.screen
                .entry(position)
                .and_modify(|pixel| {
                    if *pixel == Tile::Block {
                        self.blocks -= 1;
                    }

                    *pixel = tile;
                })
                .or_insert(tile);
        }

        Ok(self.machine.get_state())
    }

    fn present(&mut self) {
        if self.screen.is_empty() {
            return;
        }

        if self.bitmap.is_none() {
            let resolution = self
                .screen
                .keys()
                .copied()
                .reduce(|acc, next| acc.simd_max(next))
                .expect("screen is empty")
                + vec2(1, 1);
            let bitmap = Bitmap::new(resolution.x as u64, resolution.y as u64);
            self.bitmap = Some(bitmap)
        };

        let bitmap = self.bitmap.as_mut().expect("bitmap has not been set");
        for (y, x) in (0..bitmap.height()).cartesian_product(0..bitmap.width()) {
            let position = pos(x, y);
            let tile = self.screen.get(&position).copied().unwrap_or_default();
            let value = tile != Tile::Empty;
            bitmap.put(&position, value);
        }

        tracing::info!(score = self.score, screen = %OnNewLine(BoxFormatter(bitmap)));
    }
}
