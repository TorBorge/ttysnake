pub type Position = (i16, i16);

pub fn pos_add(lhs: Position, rhs: Position) -> Position {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
pub fn pos_neg(lhs: Position) -> Position {
    (-lhs.0, -lhs.1)
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

const DELTAS: [Position; 4] = [(0, 1), (2, 0), (0, -1), (-2, 0)];

impl Direction {
    pub const fn delta(self) -> Position {
        DELTAS[self as u8 as usize]
    }
}
