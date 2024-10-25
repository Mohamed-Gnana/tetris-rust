use std::time::SystemTime;

use crate::game_dynamics::GameMap;
use crate::tetrimino::entities::tetrimino::Tetrimino;
use crate::base::init::Init;
use crate::score_handler::{load_scores_and_lines, save_highscores_and_lines, update_new_achievement_vec};

pub const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
pub const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
pub struct Tetris {
    pub game_map: GameMap,
    current_level: u32,
    score: u32,
    number_of_lines: u32,
    pub current_piece: Option<Tetrimino>,
    pub next_piece: Option<Tetrimino>,
}

impl Tetris {
    pub fn create_new_piece() -> Tetrimino {
        Tetrimino::new()
    }
    pub fn remove_complete_lines(&mut self) {
        let mut score_to_be_added: u32 = 0;
        let mut y = 0;
        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break
                }
            }

            if complete {
                self.game_map.remove(y);
                score_to_be_added += self.current_level;
                y -= 1;
            }
            y += 1
        }

        if self.game_map.len() == 0 {
            println!("Tetris!!!");
            println!("Bingooooooooooooo!!");
            score_to_be_added += 1000;
        }

        self.update_score(score_to_be_added);

        while self.game_map.len() < 16 {
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    pub fn stick_current_piece(&mut self) {
        let mut score_to_be_added = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;
            while shift_y < piece.states[piece.current_state as usize].len() &&
                    (piece.y + shift_y as usize) < self.game_map.len() {
                let mut shift_x = 0;
                while shift_x < piece.states[piece.current_state as usize][shift_y].len() &&
                        (piece.x + shift_x as isize) < self.game_map[piece.y + shift_y].len() as isize {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] = piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            score_to_be_added += self.current_level;
        }
        self.update_score(score_to_be_added);
        self.remove_complete_lines();
        self.current_piece = None;
    }

    pub fn update_score(&mut self, score: u32) {
        self.score += score;
    }

    pub fn increase_lines(&mut self) {
        self.number_of_lines += 1;
        if self.number_of_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.current_level += 1;
        }
    }

    pub fn is_level_time_over(&mut self, timer: &mut SystemTime) -> bool {
        match timer.elapsed() {
            Ok(elapsed) => {
                let millie =  elapsed.as_secs() as u32 * 1000 + 
                elapsed.subsec_nanos() as u32 / 1_000_000;
                millie > LEVEL_TIMES[self.current_level as usize - 1]
            }
            Err(_) => false
        }
    }

    pub fn print_game_info(&self) -> Vec<String> {
        let mut new_highest_score = true;
        let mut new_highest_lines = true;
        if let Some((mut highscores, mut lines)) = load_scores_and_lines(None) {
            new_highest_score = update_new_achievement_vec(&mut highscores, self.score);
            new_highest_lines = update_new_achievement_vec(&mut lines, self.number_of_lines);

            if new_highest_lines || new_highest_score {
                save_highscores_and_lines(&highscores, &lines, None);
            }
        }
        vec!(
        format!("Score: {}{}", self.score, if new_highest_score { "[HIGHEST]" } else { "" }),
        format!("Level: {}{}", self.current_level, if new_highest_lines { "[HIGHEST]" } else { "" }),
        format!("Line : {}{}", self.number_of_lines, if new_highest_lines { "[HIGHEST]" } else { "" }),)
    }
}

impl Init<Tetris> for Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }

        Tetris {
            game_map: game_map,
            current_level: 1,
            score: 0,
            number_of_lines: 0,
            current_piece: None,
            next_piece: None
        }
    }
}