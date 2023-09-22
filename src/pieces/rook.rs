use crate::board::Board;
use crate::pieces::pieces::{Piece, position_on_board};
use crate::game::{Game, Player};

pub struct Rook {
    pub player: Player,
}
impl Rook {
    fn valid_move(&self, b: &Board, row:i32, col:i32) -> bool{
        if !position_on_board(row, col)  || *b.get_piece((row * 8 + col) as usize).get_player() == self.player {
            return false;
        }
        return true;
    }
    fn add_valid_move(&self, b: &Board, row: i32, col: i32, valid_moves: &mut Vec<[i32; 2]> ) -> bool {
        return if !self.valid_move(b, row, col) {
            false
        } else if Game::opposite_player(&self.player, b.get_piece((row * 8 + col) as usize).get_player()) {
            valid_moves.push([row, col]);
            false
        } else {
            valid_moves.push([row, col]);
            true
        }
    }
}

impl Piece for Rook {
    fn get_moves(&self, b: &Board, position: i32) ->  Vec<[i32; 2]> {
        let start_r = position / 8;
        let start_c = position % 8;
        let mut curr_r = start_r;
        let mut curr_c = start_c + 1;
        let mut valid_moves = Vec::new();
        while curr_c < 8 {
            if !self.add_valid_move(b, curr_r, curr_c, &mut valid_moves) {
                break;
            }
            curr_c += 1;

        }
        curr_r = start_r;
        curr_c = start_c - 1;
        while curr_c >= 0 {
            if !self.add_valid_move(b, curr_r, curr_c, &mut valid_moves) {
                break;
            }
            curr_c -= 1;
        }
        curr_r = start_r + 1;
        curr_c = start_c;
        while curr_r < 8 {
            if !self.add_valid_move(b, curr_r, curr_c, &mut valid_moves) {
                break;
            }
            curr_r += 1;
        }
        curr_r = start_r - 1;
        curr_c = start_c;
        while curr_r >= 0 {
            if !self.add_valid_move(b, curr_r, curr_c, &mut valid_moves) {
                break;
            }
            curr_r -= 1;
        }
        return valid_moves;
    }
    fn get_name(&self) -> &str {
        "R"
    }
    fn get_player(&self) -> &Player {
        &self.player
    }
    fn clone_piece(&self) -> Box<dyn Piece> {
        return Box::new(Rook {
            player: self.player
        })
    }
}