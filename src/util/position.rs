use super::vector::{vec2, Vec2};

pub type Position = Vec2;

pub fn pos(x: i64, y: i64) -> Position {
    vec2(x, y)
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => {
                self.y = self.y.wrapping_sub(1);
            }
            Direction::Down => {
                self.y = self.y.wrapping_add(1);
            }
            Direction::Left => {
                self.x = self.x.wrapping_sub(1);
            }
            Direction::Right => {
                self.x = self.x.wrapping_add(1);
            }
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Movement> for Position {
    fn add_assign(&mut self, rhs: Movement) {
        match rhs.direction {
            Direction::Up => {
                self.y = self.y.wrapping_sub(rhs.distance as i64);
            }
            Direction::Down => {
                self.y = self.y.wrapping_add(rhs.distance as i64);
            }
            Direction::Left => {
                self.x = self.x.wrapping_sub(rhs.distance as i64);
            }
            Direction::Right => {
                self.x = self.x.wrapping_add(rhs.distance as i64);
            }
        }
    }
}

impl std::ops::Add<Movement> for Position {
    type Output = Self;

    fn add(mut self, rhs: Movement) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::SubAssign<Direction> for Position {
    fn sub_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => {
                self.y = self.y.wrapping_add(1);
            }
            Direction::Down => {
                self.y = self.y.wrapping_sub(1);
            }
            Direction::Left => {
                self.x = self.x.wrapping_add(1);
            }
            Direction::Right => {
                self.x = self.x.wrapping_sub(1);
            }
        }
    }
}

impl std::ops::Sub<Direction> for Position {
    type Output = Self;

    fn sub(mut self, rhs: Direction) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign<Movement> for Position {
    fn sub_assign(&mut self, rhs: Movement) {
        match rhs.direction {
            Direction::Up => {
                self.y = self.y.wrapping_add(rhs.distance as i64);
            }
            Direction::Down => {
                self.y = self.y.wrapping_sub(rhs.distance as i64);
            }
            Direction::Left => {
                self.x = self.x.wrapping_add(rhs.distance as i64);
            }
            Direction::Right => {
                self.x = self.x.wrapping_sub(rhs.distance as i64);
            }
        }
    }
}

impl std::ops::Sub<Movement> for Position {
    type Output = Self;

    fn sub(mut self, rhs: Movement) -> Self::Output {
        self -= rhs;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub const fn inverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub const fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub const fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    pub const fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
        }
    }

    pub const fn orientation(&self) -> Orientation {
        match self {
            Self::Up => Orientation::Vertical,
            Self::Down => Orientation::Vertical,
            Self::Left => Orientation::Horizontal,
            Self::Right => Orientation::Horizontal,
        }
    }
}

impl std::ops::Mul<i64> for Direction {
    type Output = Movement;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::Output {
            direction: if rhs < 0 { self.inverse() } else { self },
            distance: rhs.unsigned_abs(),
        }
    }
}

impl std::ops::Mul<u64> for Direction {
    type Output = Movement;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::Output {
            direction: self,
            distance: rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement {
    pub direction: Direction,
    pub distance: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub const ALL: [Self; 2] = [Self::Horizontal, Self::Vertical];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    pub const ALL: [Self; 2] = [Self::Left, Self::Right];
}
