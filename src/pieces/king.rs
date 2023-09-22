use crate::board::Board;
use crate::game::Player;
use crate::pieces::pieces::Piece;
use crate::pieces::pieces::position_on_board;

pub struct King {
    pub player: Player,

}

impl King {
    fn valid_move(&self, b: &Board, row: i32, col: i32) -> bool{
        if !position_on_board(row, col)  || *b.get_piece((row * 8 + col) as usize).get_player() == self.player {
            return false;
        }
        return true;
    }
}

impl Piece for King {
    fn get_moves(&self, b: &Board, position: i32) ->  Vec<[i32; 2]> {
        let curr_x = position / 8;
        let curr_y = position % 8;
        let mut valid_moves = Vec::new();
        for row_offset in [-1, 0, 1]  {
            for col_offset in [-1, 0, 1]  {
                let candidate_pos_x = row_offset + curr_x;
                let candidate_pos_y = col_offset + curr_y;
                if row_offset == 0 && col_offset == 0 || !self.valid_move(b, candidate_pos_x, candidate_pos_y){
                    continue;
                } else {
                    valid_moves.push([candidate_pos_x, candidate_pos_y])
                }
            }
        }
        return valid_moves;
    }

    fn get_name(&self) -> &str {
        "K"
    }
    fn get_player(&self) -> &Player {
        &self.player
    }
    fn clone_piece(&self) -> Box<dyn Piece> {
        return Box::new(King {
            player: self.player
        })
    }
}