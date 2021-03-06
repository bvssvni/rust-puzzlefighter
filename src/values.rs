extern crate uuid;
extern crate rand;

use self::uuid::Uuid;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    x: i8,
    y: i8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PixelPosition {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dimension {
    w: u32,
    h: u32,
}

impl Dimension {
    pub fn new(w: u32, h: u32) -> Self {
        Dimension {
            w: w,
            h: h,
        }
    }

    pub fn from_tuple(tuple: (u32, u32)) -> Self {
        Dimension::new(tuple.0, tuple.1)
    }

    pub fn w(&self) -> u32 { self.w }
    pub fn h(&self) -> u32 { self.h }
}

impl PixelPosition {
    pub fn new(x: u32, y: u32) -> Self {
        PixelPosition {
            x: x,
            y: y,
        }
    }

    pub fn x(&self) -> u32 { self.x }
    pub fn y(&self) -> u32 { self.y }

    pub fn add(&self, rhs: Self) -> Self {
        PixelPosition::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl GridPosition {
    pub fn new(x: i8, y: i8) -> Self {
        GridPosition {
            x: x,
            y: y,
        }
    }

    pub fn offset(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up    => { GridPosition { x: self.x, y: self.y + 1 }},
            Direction::Down  => { GridPosition { x: self.x, y: self.y - 1 }},
            Direction::Left  => { GridPosition { x: self.x - 1, y: self.y }},
            Direction::Right => { GridPosition { x: self.x + 1, y: self.y }},
        }
    }

    pub fn x(&self) -> i8 { self.x }
    pub fn y(&self) -> i8 { self.y }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    id: Uuid,
    pub color: Color,
    breaker: bool,
}

impl Block {
    pub fn breaker(&self) -> bool { self.breaker }

    pub fn new(color: Color, breaker: bool) -> Self {
        Block {
            id: Uuid::new_v4(),
            color: color,
            breaker: breaker,
        }
    }

    pub fn to_texture_name(&self) -> &'static str {
        if self.breaker {
            match self.color {
                Color::Blue   => "element_blue_polygon.png",
                Color::Red    => "element_red_polygon.png",
                Color::Green  => "element_green_polygon.png",
                Color::Yellow => "element_yellow_polygon.png",
            }
        } else {
            match self.color {
                Color::Blue   => "element_blue_square.png",
                Color::Red    => "element_red_square.png",
                Color::Green  => "element_green_square.png",
                Color::Yellow => "element_yellow_square.png",
            }
        }
    }

}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Block {}
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec!(
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left
        )
    }
    pub fn clockwise(&self) -> Self {
        match *self {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        }
    }

    pub fn anti_clockwise(&self) -> Self {
        match *self {
            Direction::Up    => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Down,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    // TODO: These shouldn't be public
    pub blocks: [Block; 2],
    pub direction: Direction,
    pub position: GridPosition,
}

impl Piece {
    pub fn rand(x: i8, y: i8) -> Self {
        use self::rand::*;

        let mut rng = thread_rng();
        let pos = GridPosition::new(x, y);
        let block1 = Block::new(Color::rand(), rng.gen_weighted_bool(4));
        let block2 = Block::new(Color::rand(), rng.gen_weighted_bool(4));

        Piece {
            blocks: [block1, block2],
            position: pos,
            direction: Direction::Up,
        }
    }

    pub fn dup_to(&self, position: GridPosition, direction: Direction) -> Self {
        Piece {
            blocks: self.blocks,
            position: position,
            direction: direction,
        }
    }

    // Return blocks with positions, bottom to top ordered.
    pub fn blocks(&self) -> [PositionedBlock; 2] {
        let position = self.position;

        let positions = [
            PositionedBlock::new(self.blocks[0], position),
            PositionedBlock::new(self.blocks[1], position.offset(self.direction)),
        ];
        match self.direction {
            Direction::Down => { [positions[1], positions[0]] },
            _               => { positions },
        }
    }

    pub fn offset(&self, direction: Direction) -> Self {
        let position = self.position.offset(direction);

        Piece {
            position: position,
            ..*self
        }
    }

    pub fn clockwise(&self) -> Self {
        let direction = self.direction.clockwise();

        Piece {
            direction: direction,
            ..*self
        }
    }

    pub fn anti_clockwise(&self) -> Self {
        let direction = self.direction.anti_clockwise();

        Piece {
            direction: direction,
            ..*self
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PositionedBlock {
    block: Block,
    position: GridPosition,
}

impl PositionedBlock {
    pub fn new(block: Block, position: GridPosition) -> Self {
        PositionedBlock {
            block: block,
            position: position,
        }
    }

    pub fn x(&self) -> i8 { self.position.x() }
    pub fn y(&self) -> i8 { self.position.y() }
    pub fn block(&self) -> Block { self.block }
    pub fn position(&self) -> GridPosition { self.position }
    pub fn color(&self) -> Color { self.block.color }
    pub fn breaker(&self) -> bool { self.block.breaker() }

    pub fn offset(&self, direction: Direction) -> Self {
        let position = self.position.offset(direction);

        PositionedBlock {
            position: position,
            ..*self
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow
}

impl Color {
    pub fn rand() -> Self {
        use self::rand::*;

        let all = vec![
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Yellow
        ];
        let mut rng = rand::thread_rng();

        *rng.choose(&all).unwrap()
    }
}

