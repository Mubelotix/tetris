use wasm_game_lib::graphics::drawable::Drawable;
use wasm_game_lib::graphics::canvas::Canvas;
use wasm_game_lib::graphics::image::Image;

use crate::square::Color;

pub struct Grid<'a> {
    pub textures: [&'a Image; 7],
    pub grid: [Vec<Color>; 10],
    pub completed_lines: usize,
    pub score: usize
}

impl<'a> Grid<'a> {
    pub fn delete_completed_lines(&mut self) {
        let mut completed_lines_last_turn = 0;
        for y in 0..20 {
            let mut void_presend = false;
            for x in 0..10 {
                if self.grid[x][y] == Color::Void {
                    void_presend = true;
                    break;
                }
            }
            if !void_presend {
                completed_lines_last_turn += 1;
                for x in 0..10 {
                    self.grid[x].remove(y);
                    self.grid[x].insert(0, Color::Void);
                }
            }
        }
        self.update_score(completed_lines_last_turn);
        self.completed_lines += completed_lines_last_turn;
    }

    pub fn level(&self) -> usize {
        let mut needed = 4;
        let mut progression = 8;
        let mut level = 1;
        while self.completed_lines > needed {
            needed += progression;
            progression += 4;
            level += 1;
        }
        level
    }

    fn update_score(&mut self, completed_lines_last_turn: usize) {
        let level = self.level();
        self.score += match completed_lines_last_turn {
            1 => 40*level,
            2 => 100*level,
            3 => 300*level,
            4 => 1200*level,
            _ => 0,
        };
    }
}

impl<'a> Drawable for Grid<'a> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        for x in 0..10 {
            for y in 0..20 {
                match self.grid[x][y] {
                    Color::Void => (),
                    Color::Blue => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[0]),
                    Color::Cyan => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[1]),
                    Color::Green => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[2]),
                    Color::Orange => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[3]),
                    Color::Purple => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[4]),
                    Color::Red => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[5]),
                    Color::Yellow => canvas.draw_image(((x*50+50) as f64, (y*50) as f64), self.textures[6]),
                }
            }
        }
    }
}