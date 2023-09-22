use crate::pieces::pieces::{Piece, position_on_board};
use crate::board::Board;
use crate::game::{Game, Player};

pub struct Pawn {
    pub player: Player,
}

impl Piece for Pawn {
    fn get_moves(&self, b: &Board, position: i32) ->  Vec<[i32; 2]> {
        let curr_r = position / 8;
        let curr_c = position % 8;
        let mut valid_moves = Vec::new();
        let capture_squares: [[i32;2]; 2];
        let mut push_squares= Vec::new();
        if self.player == Player::Black {
            capture_squares = [[curr_r + 1, curr_c + 1], [curr_r + 1, curr_c - 1]];
            push_squares.push([curr_r + 1, curr_c]);
            if curr_r == 1 {
                push_squares.push([curr_r + 2, curr_c]);
            }


        } else {
            capture_squares = [[curr_r - 1, curr_c + 1], [curr_r - 1, curr_c - 1]];
            push_squares.push([curr_r - 1, curr_c]);
            if curr_r == 6 {
                push_squares.push([curr_r - 2, curr_c]);
            }
        }
        for capture_square in capture_squares {
            if position_on_board(capture_square[0], capture_square[1]) &&
                Game::opposite_player(b.get_piece((capture_square[0] * 8 + capture_square[1]) as usize).get_player(), self.get_player()) {
                valid_moves.push([capture_square[0], capture_square[1]])
            }
        }
        for push_square in push_squares {
            if !position_on_board(push_square[0], push_square[1])  ||
                *b.get_piece((push_square[0] * 8 + push_square[1]) as usize).get_player() != Player::Empty{
                break;
            }
            valid_moves.push([push_square[0], push_square[1]])
        }

        return valid_moves;

    }
    fn get_name(&self) -> &str {
        "P"
    }
    fn get_player(&self) -> &Player {
        &self.player
    }
    fn clone_piece(&self) -> Box<dyn Piece> {
        return Box::new(Pawn {
            player: self.player
        })
    }
}