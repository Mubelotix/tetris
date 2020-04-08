use wasm_game_lib::graphics::image::Image;
use wasm_game_lib::graphics::drawable::Drawable;
use wasm_game_lib::graphics::canvas::Canvas;
use wasm_game_lib::system::log;

use crate::grid::Grid;
use web_sys::{window};

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Void,
    Blue,
    Cyan,
    Green,
    Orange,
    Purple,
    Red,
    Yellow
}

pub enum Direction {
    None,
    Down,
    Right,
    Left
}

pub struct FallingSquares<'a> {
    color: Color,
    textures: [&'a Image; 7],
    squares: Vec<(usize, usize)>,
    pub is_activated: bool
}

impl<'a> FallingSquares<'a> {
    pub fn generate(textures: [&Image; 7]) -> FallingSquares {
        let crypto = window().unwrap().crypto().unwrap();
        let mut random = [0; 1];
        crypto.get_random_values_with_u8_array(&mut random).unwrap();

        let (squares, color) = match random[0] % 7 {
            0 => (vec![(4,0), (5,0), (5,1), (6,1)], Color::Blue),
            1 => (vec![(4,0), (4,1), (5,1), (6,1)], Color::Cyan),
            2 => (vec![(6,0), (5,0), (5,1), (4,1)], Color::Green),
            3 => (vec![(4,0), (5,0), (4,1), (5,1)], Color::Orange),
            4 => (vec![(6,0), (5,0), (4,0), (3,0)], Color::Purple),
            5 => (vec![(5,0), (6,1), (4,1), (5,1)], Color::Red),
            _ => (vec![(5,0), (5,1), (4,1), (3,1)], Color::Yellow),
        };

        FallingSquares {
            textures,
            color,
            squares,
            is_activated: false
        }
    }

    pub fn can_move_in_direction(&self, direction: Direction, grid: &Grid) -> bool {
        for (x, y) in &self.squares {
            if !match direction {
                Direction::Right => *x < 9 && grid.grid[*x+1][*y] == Color::Void,
                Direction::Left => *x > 0 && grid.grid[*x-1][*y] == Color::Void,
                Direction::Down => *y < 19 && grid.grid[*x][*y+1] == Color::Void,
                Direction::None => *x <= 9 && *y <= 19 && grid.grid[*x][*y] == Color::Void,
            } {
                return false;
            }
        }

        true
    }

    pub fn apply_on_grid(&self, grid: &mut Grid) {
        for (x, y) in &self.squares {
            grid.grid[*x][*y] = self.color;
        }
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Down => {
                for (_x, y) in &mut self.squares {
                    *y += 1;
                }
            },
            Direction::Left => {
                for (x, _y) in &mut self.squares {
                    *x -= 1;
                }
            },
            Direction::Right => {
                for (x, _y) in &mut self.squares {
                    *x += 1;
                }
            },
            Direction::None => ()
        }
    }

    pub fn try_rotate(&mut self, grid: &Grid) {
        use std::convert::TryInto;

        let mut xmin = 9;
        let mut xmax = 0;
        let mut ymin = 19;
        let mut ymax = 0;

        for (x, y) in &self.squares {
            if *x > xmax {
                xmax = *x;
            }
            if *x < xmin {
                xmin = *x;
            }
            if *y > ymax {
                ymax = *y;
            }
            if *y < ymin {
                ymin = *y;
            }
        }

        let mut new_squares = Vec::new();
        let mut error = false;
        for (mut x_before, mut y_before) in &self.squares {
            x_before -= xmin;
            y_before -= ymin;
            let mut y_after = x_before;
            let mut x_after = (xmax - xmin) as i32 - y_before as i32;
            x_after += xmin as i32;
            if let Ok(x_after) = x_after.try_into() {
                y_after += ymin;
                new_squares.push((x_after, y_after)); 
            } else {
                error = true;
                break;
            }
        }

        if !error {
            let old_squares = self.squares.clone();
            self.squares = new_squares;
            if !self.can_move_in_direction(Direction::None, grid) {
                self.squares = old_squares;
            }
        }
    }
}

impl<'a> Drawable for FallingSquares<'a> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        let offset_x = if self.is_activated {
            50
        } else {
            600
        };

        let offset_y = if self.is_activated {
            0
        } else {
            50
        };
        
        for (x, y) in &self.squares {
            match self.color {
                Color::Void => (),
                Color::Blue => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[0]),
                Color::Cyan => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[1]),
                Color::Green => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[2]),
                Color::Orange => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[3]),
                Color::Purple => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[4]),
                Color::Red => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[5]),
                Color::Yellow => canvas.draw_image(((x*50 + offset_x) as f64, (y*50 + offset_y) as f64), self.textures[6]),
            }
        }
    }
}

