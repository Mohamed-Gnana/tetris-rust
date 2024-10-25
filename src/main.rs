pub mod tetrimino;
pub mod game_dynamics;
pub mod tetris;
pub mod base;
pub mod file_handler;
pub mod score_handler;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::{ Canvas, Texture, TextureCreator };
use sdl2::rect::Rect;
use sdl2::ttf::FontStyle;
use sdl2::video::{Window, WindowContext};
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::ttf::Font;
use tetrimino::entities::tetrimino::Tetrimino;
use tetrimino::traits::movement::Movement;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use crate::tetris::{entities::Tetris, TETRIS_HIGHT, TETRIS_WIDTH};
use crate::base::init::Init;
use crate::game_dynamics::events::handle_events;


const TEXTURE_SIZE: u32 = 32;
const WINDOW_HIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 1200;

fn main() {
    let sdl_context = sdl2::init().expect("SDL Initialization Failed");
    //let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("SDL Image initialization failed");
    let ttf_context = sdl2::ttf::init().expect("Failed to load ttf");
    let mut font = ttf_context.load_font("assets/ToThePointRegular-n9y4.ttf", 128).expect("Failed to load the font");
    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    let grid_x = 20;
    let grid_y = (WINDOW_HIGHT - TETRIS_HIGHT * 16) as i32 / 2;
    
    let video_subsystem = sdl_context.video().expect("Video subsystem initialization failed.");
    let window = video_subsystem
                .window("rust-sdl2 demo: Video", WINDOW_WIDTH, WINDOW_HIGHT)
                .position_centered()
                .opengl()
                .build()
                .expect("Failed to create window");

    let mut canvas = window
                    .into_canvas()
                    .target_texture()
                    .present_vsync()
                    .build()
                    .expect("Couldn't convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    //let image_texture = texture_creator.load_texture("assets/IMG_20190910_174553.jpg").expect("Couldn't load the image");

    let grid = create_texture_rect(&mut canvas, &texture_creator, 0, 0, 0, TETRIS_HIGHT as u32 * 10, TETRIS_WIDTH as u32 * 16).expect("Failed to create texture.");
    let border = create_texture_rect(&mut canvas, &texture_creator, 255, 255, 255, TETRIS_HIGHT as u32 * 10 + 20, TETRIS_WIDTH as u32 * 16 + 20).expect("Failed to create texture.");
    
    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => {
            create_texture_rect(&mut canvas,
            &texture_creator,
            $r, $g, $b,
            TETRIS_HIGHT as u32,
            TETRIS_WIDTH as u32).unwrap()
        };
    }

    let textures = [texture!(255, 69, 69), texture!(255, 220, 69),
                                      texture!(237, 150, 37), texture!(171, 99, 237), texture!(77, 149, 239),
                                      texture!(39, 218, 225), texture!(45, 216, 47)];

    let mut event_pump = sdl_context.event_pump().expect("failed to get event_pump");
    loop {
        if tetris.is_level_time_over(&mut timer){
            let mut make_permanent = false;

            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.move_position(&tetris.game_map, x, y);
            }

            if make_permanent {
                tetris.stick_current_piece();
            }

            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        //canvas.copy(&image_texture, None, None).expect("Failed to render the image");

        canvas.copy(&border, 
                        None, 
                        Rect::new(10,
                                       ((WINDOW_HIGHT - TETRIS_HIGHT * 16) as i32 / 2) - 10,
                                       TETRIS_WIDTH * 10 + 20, TETRIS_HIGHT * 16 + 20))
             .expect("Failed to copy border into window");      
        canvas.copy(&grid, 
            None, 
            Rect::new(20,
                            ((WINDOW_HIGHT - TETRIS_HIGHT * 16) as i32 / 2),
                            TETRIS_WIDTH * 10, TETRIS_HIGHT * 16))
                .expect("Failed to copy grid into window");



        if tetris.current_piece.is_none() {
            let mut current_piece = Tetris::create_new_piece();
            tetris.current_piece = tetris.next_piece.take();

            if(tetris.current_piece.is_none()) {
                if !current_piece.has_valid_current_position(&tetris.game_map) {
                    tetris.print_game_info();
                    break
                }
                tetris.current_piece = Some(current_piece);
            }
        }

        if tetris.next_piece.is_none() {
            let mut piece = Tetris::create_new_piece();
            if !piece.has_valid_current_position(&tetris.game_map) {
                tetris.print_game_info();
                break
            }
            tetris.next_piece = Some(piece);
        }

        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                for (line_nb, line) in piece.states[piece.current_state as usize].iter().enumerate() {
                    for (case_nb, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue
                        }

                        canvas.copy(&textures[*case as usize - 1], 
                                            None, 
                                                Rect::new(grid_x + (piece.x + case_nb as isize) as i32 * TETRIS_WIDTH as i32,
                                                                grid_y + (piece.y + line_nb) as i32 * TETRIS_HIGHT as i32,
                                                                TETRIS_WIDTH,
                                                            TETRIS_HIGHT))
                              .expect("Failed to render the tetrimino");  
                    }
                }
            }
        }

        if quit {
            tetris.print_game_info();
            break
        }

        


        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue
                }

                canvas.copy(&textures[*case as usize - 1], 
                    None, 
                        Rect::new(grid_x + case_nb as i32 * TETRIS_WIDTH as i32,
                                        grid_y + line_nb as i32 * TETRIS_HIGHT as i32,
                                        TETRIS_WIDTH,
                                    TETRIS_HIGHT))
                    .expect("Failed to render the game tetrimino");  
            }
        }

        display_game_info(&mut tetris, &mut canvas, &texture_creator, &font, WINDOW_WIDTH as i32 - grid_x - 325, 255, 255, 255);

        draw_next_piece(&textures, &tetris, &mut canvas, &texture_creator, 0, 0, 0, TETRIS_HIGHT as u32, TETRIS_WIDTH as u32, WINDOW_WIDTH as i32 - grid_x - 350, 320);
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32));
    }
}

fn draw_next_piece<'a>(
    textures: &[Texture<'_>],
    tetris: &Tetris,
    canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>, 
    red: u8, green: u8, blue: u8, 
    height: u32, width: u32,
    x: i32, y: i32) {
        if let Some(ref piece) = tetris.next_piece {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            let rect = Rect::new(x, y, width * 5, height * 5);
            canvas.fill_rect(rect).expect("Failed to draw the next piece");
            let cell_width = rect.width() / 4; // Adjust for a 4x4 grid size
            let cell_height = rect.height() / 4;
            for (line_nb, line) in piece.states[piece.current_state as usize].iter().enumerate() {
                for (case_nb, case) in line.iter().enumerate() {
                    if *case == 0 {
                        continue
                    }

                    let texture = &textures[*case as usize - 1];
                    let texture_query = texture.query();
                    // Define destination rectangle within the cell grid
                    let dest_rect = Rect::new(
                        rect.x + (case_nb as i32 * cell_width as i32),
                        rect.y + (line_nb as i32 * cell_height as i32),
                        cell_width.min(texture_query.width * 5),
                        cell_height.min(texture_query.height  * 5),
                    );
                    canvas.copy(texture, 
                                    None, 
                                    Some(dest_rect))
                        .expect("Failed to render the tetrimino");  
                }
            }
        } 
}


fn display_game_info<'a>(tetris: &Tetris,
    canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font,
    start_point: i32,
    red: u8, green: u8, blue: u8) {
        let infos = tetris.print_game_info();
        let mut y = 10;
        for info in infos {
            let texture = create_texture_from_text(canvas, texture_creator, font, &info, red, green, blue).expect("Failed to create text");
            canvas.copy(&texture, None, get_text_from_rect(&info, start_point, y)).expect("Couldn't render font");
            y += 25;
        }
    }

// fn convert_texts_to_texture<'a>(data: &[String],
//     canvas: &mut Canvas<Window>, 
//     texture_creator: &'a TextureCreator<WindowContext>,
//     font: &Font,
//     text: &str,
//     red: u8, green: u8, blue: u8) -> Vec<Texture<'a>> {
//         data.iter().filter_map(|x| {
//             if let Some(texture) = create_texture_from_text(canvas, texture_creator, font, x.as_str(), red, green, blue) {
//                 return Some(texture);
//             }
//             else {
//                 None
//             }
//         }).collect()
// }

fn get_text_from_rect(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 20, 30))
}
fn create_texture_from_text<'a>(canvas: &mut Canvas<Window>, 
                                texture_creator: &'a TextureCreator<WindowContext>,
                                font: &Font,
                                text: &str,
                                red: u8, green: u8, blue: u8) -> Option<Texture<'a>> {
        if let Ok(surface) = font.render(text)
                                 .blended(Color::RGB(red, green, blue)) {
                                    texture_creator.create_texture_from_surface(surface).ok()
                                 }
        else {
            None
        }
}

fn create_texture_rect<'a> (canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>, 
    red: u8, green: u8, blue: u8, 
    height: u32, width: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(red, green, blue));
            texture.clear();

            texture.set_draw_color(Color::RGB(0, 0, 0));
            let inner_rect = Rect::new(
            4 as i32,
            4 as i32,
            width - 2 * 4,
            height - 2 * 4,
        );
        texture.draw_rect(inner_rect).expect("Failed to draw border");
        }).expect("Failed to color the texture");
        Some(square_texture)
    }
    else {
        None
    }
}


#[derive(Clone, Copy)]
enum TextureColor {
    Green, 
    Blue,
    Red
}