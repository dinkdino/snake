use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Up    => Direction::Down,
            Down  => Direction::Up,
            Left  => Direction::Right,
            Right => Direction::Left
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Block { x: i32, y: i32 }

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake {

    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            Some(d) => self.direction = d,
            None => ()
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Up    => Block { x: last_x,     y: last_y - 1 },
            Down  => Block { x: last_x,     y: last_y + 1 },
            Left  => Block { x: last_x - 1, y: last_y },
            Right => Block { x: last_x + 1, y: last_y }
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let moving_dir = match dir {
            Some(d) => d,
            None    => self.direction
        };

        match moving_dir {
            Up    => (head_x,     head_y - 1),
            Down  => (head_x,     head_y + 1),
            Left  => (head_x - 1, head_y    ),
            Right => (head_x + 1, head_y    )
        }
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        
        for block in self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        false
    }
}
