use crate::game_dynamics::{GameMap};

pub trait Movement {
    fn move_position(&mut self, game_map: &GameMap, shift_x: isize, shift_y: usize) -> bool;
}