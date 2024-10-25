use crate::tetris::entities::Tetris;
use crate::tetrimino::traits::{rotation::Rotation, movement::Movement};
use std::time::SystemTime;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut EventPump) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *quit = true;
                    break
                }
                Event::KeyDown { keycode : Some(Keycode::Down), .. } => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                } 
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    tmp_x += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    tmp_x -= 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let x = piece.x;
                    let mut y = piece.y;
                    while piece.move_position(&tetris.game_map, x, y + 1) {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }

        if !make_permanent {
            if !piece.move_position(&tetris.game_map, tmp_x, tmp_y) && 
                tmp_y != piece.y
            {
                make_permanent = true;
            }
        }
    }

    

    if make_permanent {
        tetris.stick_current_piece();
        *timer = SystemTime::now();
    }
    make_permanent
}