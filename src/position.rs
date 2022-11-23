use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use oorandom::Rand32;
use crate::world::{GRID_CELL_SIZE, GRID_SIZE};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    pub fn default() -> Self {
        GridPosition::new(0,0)
    }

    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn random(rng: &mut Rand32, max_x: i16, max_y: i16) -> Self {
        (
            rng.rand_range(0..(max_x as u32)) as i16,
            rng.rand_range(0..(max_y as u32)) as i16,
        )
            .into()
    }

    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, (pos.y - 1).rem_euclid(GRID_SIZE.1)),
            Direction::Down => GridPosition::new(pos.x, (pos.y + 1).rem_euclid(GRID_SIZE.1)),
            Direction::Left => GridPosition::new((pos.x - 1).rem_euclid(GRID_SIZE.0), pos.y),
            Direction::Right => GridPosition::new((pos.x + 1).rem_euclid(GRID_SIZE.0), pos.y),
        }
    }

    pub fn new_from_offset(&self, offset: [i16; 2]) -> Self {
        GridPosition::new(self.x + offset[0], self.y + offset[1])
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}
