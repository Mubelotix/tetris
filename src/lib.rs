#[allow(unused_imports)]

use wasm_bindgen::{prelude::*, JsCast};
use wasm_game_lib::graphics::image::Image;
use wasm_game_lib::graphics::sprite::Sprite;
use wasm_game_lib::graphics::font::Font;
use wasm_game_lib::graphics::text::Text;
use wasm_game_lib::inputs::event::Event;
use wasm_game_lib::inputs::keyboard::*;
use wasm_game_lib::inputs::mouse::*;
use wasm_game_lib::graphics::window::Window;
use wasm_game_lib::graphics::color::Color as LibColor;
use wasm_game_lib::system::log;
use wasm_game_lib::inputs::event::types::*;
use wasm_game_lib::system::sleep;
use std::time::Duration;
use console_error_panic_hook::set_once;
use futures::join;
mod square;
mod grid;
use grid::Grid;
use square::*;
use futures::channel::oneshot;
use futures::channel::oneshot::Receiver;

#[wasm_bindgen(module = "/module.js")]
extern "C" {
    fn get_timestamp() -> u32;
}

async fn loading_tracker(mut receivers: Vec<Receiver<Result<Image, JsValue>>>) -> Vec<Image> {
    let mut images = Vec::new();
    for _ in 0..receivers.len() {
        images.push(None);
    }
 
    loop {
        for i in 0..images.len() {
            if images[i].is_none() {
                if let Ok(Some(result)) = receivers[i].try_recv() {
                    images[i] = Some(result);
                }
            }
        }
         
        if !images.contains(&None) {
            // break when every image is ready
            break;
        }
 
        // you may want to display a progress bar here
 
        sleep(Duration::from_millis(20)).await;
    }
 
    let mut unwraped_images = Vec::new();
    for image in images {
        unwraped_images.push(image.unwrap().expect("can't load one image"));
    }
 
    return unwraped_images;
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    set_once();

    let (mut window, mut canvas) = Window::init_with_events(KEYBOARD_EVENT + RESIZE_EVENT + FOCUS_EVENT + MOUSE_EVENT);
    
    let (sender1, receiver1) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender2, receiver2) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender3, receiver3) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender4, receiver4) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender5, receiver5) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender6, receiver6) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender7, receiver7) = oneshot::channel::<Result<Image, JsValue>>();
    let (sender8, receiver8) = oneshot::channel::<Result<Image, JsValue>>();

    let loading_tracker_future = loading_tracker(vec![receiver1, receiver2, receiver3, receiver4, receiver5, receiver6, receiver7, receiver8]);
    let image1_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/blue_square.png", sender1);
    let image2_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/cyan_square.png", sender2);
    let image3_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/green_square.png", sender3);
    let image4_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/orange_square.png", sender4);
    let image5_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/purple_square.png", sender5);
    let image6_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/red_square.png", sender6);
    let image7_future = Image::load_and_send("https://raw.githubusercontent.com/Mubelotix/tetris_nsi/master/textures/yellow_square.png", sender7);
    let image8_future = Image::load_and_send("background.png", sender8);

    let images = join!(loading_tracker_future, image1_future, image2_future, image3_future, image4_future, image5_future, image6_future, image7_future, image8_future).0;

    let font = Font::load("font.ttf").await.unwrap();

    let mut grid = Grid { 
        textures: [&images[0], &images[1], &images[2], &images[3], &images[4], &images[5], &images[6]],
        grid: [ vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void],
                vec![Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void, Color::Void]
        ],
        completed_lines: 0,
        score: 0
    };
    
    let mut falling_squares = FallingSquares::generate([&images[0], &images[1], &images[2], &images[3], &images[4], &images[5], &images[6]]);
    falling_squares.is_activated = true;
    let mut last_move_timestamp = 0;
    let mut lost_focus_timestamp = 0;
    let mut focus = true;
    let mut next_falling_squares = FallingSquares::generate([&images[0], &images[1], &images[2], &images[3], &images[4], &images[5], &images[6]]);

    let background = Sprite::<u32>::new((0,0), &images[7], (0,0));

    let mut lines_cleared_text = Text::new_with_text_and_coords(&font, format!("Lines: {}", grid.completed_lines), (600, 500));
    lines_cleared_text.style.color = LibColor::white();
    lines_cleared_text.character_size = (50, "px");

    let mut level_text = Text::new_with_text_and_coords(&font, format!("Level: {}", grid.level()), (600, 600));
    level_text.style.color = LibColor::white();
    level_text.character_size = (50, "px");

    let mut score_text = Text::new_with_text_and_coords(&font, format!("Score: {}", grid.score), (600, 700));
    score_text.style.color = LibColor::white();
    score_text.character_size = (50, "px");

    loop {
        let timestamp = get_timestamp();

        for event in window.poll_events() {
            match event {
                Event::KeyboardEvent(keyboard_event) => match keyboard_event {
                    KeyboardEvent::Down(key) => {
                        match key {
                            Key::LeftArrow => if falling_squares.can_move_in_direction(Direction::Left, &grid) {
                                falling_squares.move_in_direction(Direction::Left);
                            },
                            Key::RightArrow => if falling_squares.can_move_in_direction(Direction::Right, &grid) {
                                falling_squares.move_in_direction(Direction::Right);
                            },
                            Key::DownArrow => if falling_squares.can_move_in_direction(Direction::Down, &grid) {
                                falling_squares.move_in_direction(Direction::Down);
                                last_move_timestamp = timestamp;
                            } else {
                                falling_squares.apply_on_grid(&mut grid);
                                grid.delete_completed_lines();
                                lines_cleared_text.set_text(format!("Lines: {}", grid.completed_lines));
                                level_text.set_text(format!("Level: {}", grid.level()));
                                score_text.set_text(format!("Score: {}", grid.score));
                                falling_squares = next_falling_squares;
                                falling_squares.is_activated = true;
                                next_falling_squares = FallingSquares::generate([&images[0], &images[1], &images[2], &images[3], &images[4], &images[5], &images[6]]);
                            },
                            Key::UpArrow => falling_squares.try_rotate(&grid),
                            _ => (),
                        }
                    },
                    _ => (),
                },
                Event::ResizeEvent(width, height) => {
                    canvas.set_width(width);
                    canvas.set_height(height);
                    log("resizing");
                }
                Event::FocusEvent(new_focus) => {
                    focus = new_focus;

                    let timestamp = get_timestamp();
                    if !focus {
                        lost_focus_timestamp = timestamp;
                    } else {
                        last_move_timestamp = timestamp - (lost_focus_timestamp - last_move_timestamp);
                    }
                }
                Event::MouseEvent(mouse_event) => match mouse_event {
                    MouseEvent::Down(x, y) => {
                        if y < 1060 && x < 200 && falling_squares.can_move_in_direction(Direction::Left, &grid) {
                            falling_squares.move_in_direction(Direction::Left);
                        } else if y < 1060 && x > canvas.get_width() - 200 && falling_squares.can_move_in_direction(Direction::Right, &grid) {
                            falling_squares.move_in_direction(Direction::Right);
                        } else if y >= 1060 {
                            if falling_squares.can_move_in_direction(Direction::Down, &grid) {
                                falling_squares.move_in_direction(Direction::Down);
                                last_move_timestamp = timestamp;
                            } else {
                                falling_squares.apply_on_grid(&mut grid);
                                grid.delete_completed_lines();
                                lines_cleared_text.set_text(format!("Lines: {}", grid.completed_lines));
                                level_text.set_text(format!("Level: {}", grid.level()));
                                score_text.set_text(format!("Score: {}", grid.score));
                                falling_squares = next_falling_squares;
                                falling_squares.is_activated = true;
                                next_falling_squares = FallingSquares::generate([&images[0], &images[1], &images[2], &images[3], &images[4], &images[5], &images[6]]);
                            }
                        } else {
                            falling_squares.try_rotate(&grid)
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        }
        
        if focus && timestamp - last_move_timestamp > 1000 {
            if falling_squares.can_move_in_direction(Direction::Down, &grid) {
                falling_squares.move_in_direction(Direction::Down);
                last_move_timestamp = timestamp;
            } else {
                falling_squares.apply_on_grid(&mut grid);
                grid.delete_completed_lines();
                lines_cleared_text.set_text(format!("Lines: {}", grid.completed_lines));
                level_text.set_text(format!("Level: {}", grid.level()));
                score_text.set_text(format!("Score: {}", grid.score));
                falling_squares = next_falling_squares;
                falling_squares.is_activated = true;
                next_falling_squares = FallingSquares::generate([&images[0], &images[1], &images[2], &images[3], &images[4], &images[5], &images[6]]);
            }
        }

        canvas.clear_with_black();
        canvas.draw(&background);
        canvas.draw(&grid);
        canvas.draw(&lines_cleared_text);
        canvas.draw(&level_text);
        canvas.draw(&score_text);
        canvas.draw(&falling_squares);
        canvas.draw(&next_falling_squares);

        sleep(Duration::from_millis(16)).await;
    }

    Ok(())
}
