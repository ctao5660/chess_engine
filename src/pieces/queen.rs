use crate::pieces::pieces::Piece;
use crate::board::Board;
use crate::game::Player;
use crate::pieces::bishop::Bishop;
use crate::pieces::rook::Rook;

pub struct Queen {
    pub player: Player,

}

impl Piece for Queen {
    fn get_moves(&self, b: &Board, position: i32) ->  Vec<[i32; 2]> {
        // rook + bishop moves
        let player_rook: Player;
        let player_bishop: Player;
        if self.player == Player::Black {
            player_rook = Player::Black;
            player_bishop = Player::Black;
        } else {
            player_rook = Player::White;
            player_bishop = Player::White;
        }
        let rook = Rook{ player: player_rook};
        let bishop = Bishop{player: player_bishop};
        let mut total_moves = rook.get_moves(b, position);
        let mut bishop_moves = bishop.get_moves(b, position);
         total_moves.append(&mut bishop_moves);
        return total_moves;

    }
    fn get_name(&self) -> &str {
        "Q"
    }
    fn get_player(&self) -> &Player {
        &self.player
    }
    fn clone_piece(&self) -> Box<dyn Piece> {
        return Box::new(Queen {
            player: self.player
        })
    }
}