use crate::pieces::pieces::{Piece, position_on_board};
use crate::board::Board;
use crate::game::{Player};

pub struct Knight {
    pub player: Player,

}
impl Knight {
    fn valid_move(&self, b: &Board, row:i32, col:i32) -> bool{
        if !position_on_board(row, col)  || *b.get_piece((row * 8 + col) as usize).get_player() == self.player {
            return false;
        }
        return true;
    }
    fn add_valid_move(&self, b: &Board, row: i32, col: i32, valid_moves: &mut Vec<[i32; 2]> ) -> bool {
        if !self.valid_move(b, row, col) {
            return false;
        }
        valid_moves.push([row , col]);
        return true;
    }
}
impl Piece for Knight {
    fn get_moves(&self, b: &Board, position: i32) ->  Vec<[i32; 2]> {
        let curr_r = position / 8;
        let curr_c = position % 8;
        let mut valid_moves = Vec::new();
        let possible_moves = [[curr_r + 2, curr_c + 1], [curr_r + 2, curr_c - 1], [curr_r - 2, curr_c + 1],
            [curr_r - 2, curr_c - 1], [curr_r + 1, curr_c + 2], [curr_r + 1, curr_c - 2], [curr_r - 1, curr_c +2], [curr_r - 1, curr_c - 2]];
        for mv in possible_moves {
            self.add_valid_move(b, mv[0], mv[1], &mut valid_moves);
        }
        return valid_moves;
    }
    fn get_name(&self) -> &str {
        "N"
    }
    fn get_player(&self) -> &Player {
        &self.player
    }
    fn clone_piece(&self) -> Box<dyn Piece> {
        return Box::new(Knight {
            player: self.player
        })
    }
}