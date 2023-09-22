use crate::board::Board;
use crate::game::Player;

pub trait Piece {
    fn get_moves(&self, b: &Board, position: i32) -> Vec<[i32; 2]>;
    fn get_name(&self)-> &str;
    fn get_player(&self) ->&Player;
    fn clone_piece(&self) -> Box<dyn Piece>;

}

pub(crate) fn position_on_board(x_pos: i32, y_pos: i32 ) -> bool {
    if x_pos < 0 || y_pos < 0 || x_pos >= 8 || y_pos >= 8 {
        return false;
    }
    return true;
}

pub struct Empty {
    pub player: Player,
}

impl Piece for Empty {
    fn get_moves(&self, _b: &Board, _position: i32) -> Vec<[i32; 2]> {
        return Vec::new();
    }
    fn get_name(&self) -> &str {
        "_"
    }
    fn get_player(&self) -> &Player {
        &self.player
    }
    fn clone_piece(&self) -> Box<dyn Piece> {
        return Box::new(Empty {
            player: self.player
        })
    }

}