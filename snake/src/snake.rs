/**This module handles the actions of the snake
 */

use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// The possible directions
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {

    /* Check that the snake is not going to the
     * opposite direction.
     */
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

/* Snake struct
 * Direction: direction of the snake
 * body: Where the snakeblocks are stored
 * tail 
 */
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,   
}

impl Snake {
    //Creates the new snake
    pub fn new(x: i32, y: i32) -> Snake {

        // Create the linked list and push back the first blocks to it
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x+2,
            y,
        });
        body.push_back(Block {
            x: x+1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });
        
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // Draws the blocks of the snake
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
        draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // Give the heads position
    pub fn head_position(&self) -> (i32, i32) {
    let head_block = self.body.front().unwrap();
    (head_block.x, head_block.y)
    }

    // Moves snake forware to different directions
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) : (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y-1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y+1,
            },
            Direction::Left => Block {
                x: last_x-1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x+1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // Return the head position
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Gets the location of the next head
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {},
        }

        match moving_dir {
            Direction::Up => (head_x, head_y-1),
            Direction::Down => (head_x, head_y+1),
            Direction::Left =>  (head_x-1, head_y),
            Direction::Right => (head_x+1, head_y),
        }
    }

    // Increases tails size
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // If the tail overlaps, send true
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len()-1 {
                break;
            }
        }
        return false;
    }
}