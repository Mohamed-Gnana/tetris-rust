use crate::base::init::Init;
use crate::tetrimino::entities::tetrimino::*;
use rand::random;
pub mod tetrimino;

impl Init<Tetrimino> for Tetrimino {
    fn new() -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut random_value = random::<u8>() % 7;
        if unsafe {PREV} == random_value {
            random_value = random::<u8>() % 7;
        }
        unsafe { PREV = random_value; }
        match random_value {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!()
        }
    }
}
