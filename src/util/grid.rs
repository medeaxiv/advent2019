use super::position::{Direction, Position};

pub struct Grid<T> {
    width: i64,
    height: i64,
    entries: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: u64, height: u64, entries: Vec<T>) -> Self {
        assert_eq!(width * height, entries.len() as u64);

        Self {
            width: width as i64,
            height: height as i64,
            entries,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
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

    pub fn index(&self, position: &Position) -> Option<usize> {
        if self.contains(position) {
            Some((position.y * self.width + position.x) as usize)
        } else {
            None
        }
    }

    pub fn position(&self, index: usize) -> Position {
        Position::new(index as i64 % self.width, index as i64 / self.width)
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        self.index(position).and_then(|idx| self.entries.get(idx))
    }

    pub fn put(&mut self, position: &Position, tile: T) {
        if let Some(index) = self.index(position) {
            self.entries[index] = tile;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries.iter()
    }
}

impl<T> Grid<T>
where
    T: TileChar,
{
    pub fn to_char_grid(&self) -> Grid<char> {
        Grid {
            width: self.width,
            height: self.height,
            entries: self.entries.iter().map(|t| t.to_char()).collect(),
        }
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            entries: self.entries.clone(),
        }
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: TileChar,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "┌")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "┐")?;
        write!(f, "│")?;

        for (idx, tile) in self.entries.iter().enumerate() {
            if idx > 0 && idx % self.width as usize == 0 {
                writeln!(f, "│")?;
                write!(f, "│")?;
            }

            write!(f, "{}", tile.to_char())?;
        }

        writeln!(f, "│")?;
        write!(f, "└")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "┘")
    }
}

pub trait TileChar {
    fn to_char(&self) -> char;
}

impl TileChar for char {
    fn to_char(&self) -> char {
        *self
    }
}

impl TileChar for Direction {
    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}
