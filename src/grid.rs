use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub col_count: usize,
    pub row_count: usize,
    values: Vec<T>,
}

impl <T> Grid<T> {
    pub fn new(col_count: usize, row_count: usize, initial_value: T) -> Self 
    where 
        T: Clone
    {
        let values = vec![initial_value; col_count * row_count];
        Self { col_count, row_count, values }
    }

    pub fn from_lines<F>(lines: impl Iterator<Item = String>, mut parse: F) -> Self 
    where 
        F: FnMut(char, &Position) -> T, 
    {
        let mut values = Vec::new();
        let mut col_count = 0;
        let mut row_count = 0;
        for line in lines.filter(|l| !l.trim().is_empty()) {
            col_count = line.len();
            for (col, c) in line.chars().enumerate() {
                values.push(parse(c, &Position::new(col, row_count)));
            }
            row_count += 1;
        }

        Self { col_count, row_count, values }
    }

    pub fn at(&self, pos: &Position) -> &T {
        &self.values[pos.y * self.col_count + pos.x]
    }

    pub fn at_mut(&mut self, pos: &Position) -> &mut T {
        &mut self.values[pos.y * self.col_count + pos.x]
    }

    pub fn neighbours(&self, pos: &Position) -> Vec<Position> {
        let &Position { x, y } = pos;
        let mut n = Vec::with_capacity(4);
        if x > 0 {
            n.push(Position::new(x - 1, y));
        }
        if x + 1 < self.col_count {
            n.push(Position::new(x + 1, y));
        }
        if y > 0 {
            n.push(Position::new(x, y - 1));
        }
        if y + 1 < self.row_count {
            n.push(Position::new(x, y + 1));
        }
        n
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> {
        (0..self.row_count)
            .cartesian_product(0..self.col_count)
            .map(|(y, x)| Position::new(x, y))
    }

    pub fn is_valid(&self, pos: &Position) -> bool {
        pos.x < self.col_count && pos.y < self.row_count
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self { 
        Self { x, y }
    }

    pub fn origin() -> Self { 
        Self { x: 0, y: 0 }
    }
}