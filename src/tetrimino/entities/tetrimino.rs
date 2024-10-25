use crate::tetrimino::traits::movement::Movement;
use crate::tetrimino::traits::rotation::Rotation;
use crate::base::init::Init;
use crate::game_dynamics::GameMap;


// Tetrimino
type States = Vec<Piece>;
type Piece = Vec<Vec<u8>>;

pub struct Tetrimino {
    pub states: States,
    pub x: isize,
    pub y: usize,
    pub current_state: u8
}

impl Tetrimino {
    pub fn has_valid_position(&mut self, game_map: &GameMap, current_state: usize, x: isize, y: usize) -> bool {
        for tmp_y in 0..4 {
            for tmp_x in 0..4 {
                let x = x + tmp_x;
                if self.states[self.current_state as usize][tmp_y][tmp_x as usize] != 0 &&
                    (y + tmp_y >= game_map.len() ||
                    x < 0 || 
                    x as usize >= game_map[y + tmp_y].len() || 
                    game_map[y + tmp_y][x as usize] != 0)
                {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn has_valid_current_position(&mut self, game_map: &GameMap) -> bool {
        self.has_valid_position(game_map, self.current_state as usize, self.x, self.y as usize)
    }
}

impl Rotation for Tetrimino {
    fn rotate(&mut self, game_map: &GameMap) {
        let tmp_state = ((self.current_state as usize + 1) % self.states.len()) as u8;
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.has_valid_position(game_map, tmp_state as usize, self.x + x, self.y) {
                self.current_state = tmp_state;
                self.x += *x;
                break
            }
        }
    }
}

impl Movement for Tetrimino {
    fn move_position(&mut self, game_map: &GameMap, new_x: isize, new_y: usize) -> bool {
        if self.has_valid_position(game_map, self.current_state as usize, new_x, new_y as usize) {
            self.x = new_x;
            self.y = new_y;
            true
        }
        else {
            false
        }
    }
}
// End of Tetrimino

// TetriminoI
pub struct TetriminoI;

impl Init<Tetrimino> for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![1, 1, 1, 1],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0]],],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

// End of TetriminoI

// TetriminoJ
pub struct TetriminoJ;

impl Init<Tetrimino> for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![2, 2, 2, 0],
                              vec![2, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![2, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 0, 2, 0],
                              vec![2, 2, 2, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![2, 0, 0, 0],
                              vec![2, 0, 0, 0],
                              vec![2, 2, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}
// End of TetriminoL
pub struct TetriminoL;

impl Init<Tetrimino> for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![3, 3, 3, 0],
                              vec![0, 0, 3, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 0, 0, 0],
                              vec![0, 3, 0, 0],
                              vec![0, 3, 0, 0],
                              vec![3, 3, 0, 0]],
                         vec![vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![3, 0, 0, 0],
                              vec![3, 3, 3, 0]],
                         vec![vec![3, 3, 0, 0],
                              vec![3, 0, 0, 0],
                              vec![3, 0, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}
// TetriminoO
pub struct TetriminoO;

impl Init<Tetrimino> for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![4, 4, 0, 0],
                              vec![4, 4, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}
// End of TetriminoO

// TetriminoS
pub struct TetriminoS;

impl Init<Tetrimino> for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![0, 5, 5, 0],
                              vec![5, 5, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 5, 0, 0],
                              vec![0, 5, 5, 0],
                              vec![0, 0, 5, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}
// End of TetriminoS

// TetriminoZ
pub struct TetriminoZ;

impl Init<Tetrimino> for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![6, 6, 0, 0],
                              vec![0, 6, 6, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 0, 6, 0],
                              vec![0, 6, 6, 0],
                              vec![0, 6, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}
// End of TetriminoZ

// TetriminoT
pub struct TetriminoT;

impl Init<Tetrimino> for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![7, 7, 7, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 7, 0, 0],
                              vec![7, 7, 0, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 7, 0, 0],
                              vec![7, 7, 7, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 7, 0, 0],
                              vec![0, 7, 7, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}
// End of TetriminoT


