use crate::game_dynamics::{GameMap};
pub trait Rotation {
    fn rotate(&mut self, game_map: &GameMap);
}