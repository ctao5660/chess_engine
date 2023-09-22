use std::rc::Rc;
use crate::game::Player;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::pieces::{Empty, Piece};
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
pub struct Board {
    pub board:  Vec<Rc<dyn Piece>>
}


impl Board {

    pub fn new() -> Board{
        let mut board:Vec<Rc<dyn Piece>> = Vec::with_capacity(64);
        for i in 0..64 {
            board.push(Rc::new(Empty{player:Player::Empty}))
        }
        return Board{
            board
        }
    }
    pub fn get_piece(&self, index: usize) -> &dyn Piece {
        &*self.board[index]
    }
    pub fn place_piece(&mut self, index: usize, prev_index: usize, piece: &Rc<dyn Piece>) {
        self.board[index] = piece.clone();
        self.board[prev_index] = Rc::new(Empty{player: Player::Empty})

    }

    pub fn coordinate_to_idx(coord: &str) -> usize {
        let mut row: usize = 0;
        let mut col: usize = 0;
        for (i,char) in coord.chars().enumerate() {
            match i {
                0 => col = ((char as u32) - ('a' as u32)) as usize,
                1 => row = char.to_digit(10).unwrap() as usize,
                _ => panic!("invalid char {:?}", coord.chars())
            }
        }
        return (8 - row) * 8 + col
    }
    pub fn grid_to_coordinate(grid: [i32; 2]) -> String {
        let chars = ["a", "b", "c", "d" , "e", "f", "g", "f"];
        return format!("{}{}", chars[grid[1] as usize], char::from_digit(8 - grid[0] as u32, 10).unwrap())
    }

    /*pub fn place_piece(&mut self, piece: &'a dyn Piece, x_coord: i32, y_coord: i32) {
        self.board[(x_coord * 8 + y_coord) as usize] = piece;
    }*/
    pub fn print_board(board: &Vec<Rc<dyn Piece>>) {
        for i in 0..64{
            if i % 8 == 0 {
                println!()
            }
            print!("{} ", board[i as usize].get_name())
        }
    }
    pub fn board_from_fen(fen: &str) -> (Board, Vec<Rc<dyn Piece>>, Vec<Rc<dyn Piece>>){
        let mut board = Board::new();
        let pieces = Board::place_pieces_from_fen(&mut board, fen);

        return (board, pieces.0, pieces.1);

    }
    fn place_pieces_from_fen(board: &mut Board, piece_string: &str) -> (Vec<Rc<dyn Piece>>, Vec<Rc<dyn Piece>>){
        let mut curr_idx:usize = 0;
        let mut black_pieces = Vec::new();
        let mut white_pieces = Vec::new();
        for c in piece_string.chars() {
            if c.is_digit(10) {
                curr_idx += c.to_digit(10).unwrap() as usize;
                continue;
            }
            let mut curr_piece:Rc<dyn Piece> = Rc::new(Empty{player:Player::Empty});
            match c {
                'r' => curr_piece = Rc::new(Rook{player: Player::Black}),
                'n' => curr_piece = Rc::new(Knight{player: Player::Black}),
                'b' => curr_piece = Rc::new(Bishop{player: Player::Black}),
                'q' => curr_piece = Rc::new(Queen{player: Player::Black}),
                'k' => curr_piece = Rc::new(King{player: Player::Black}),
                'p' => curr_piece = Rc::new(Pawn{player: Player::Black}),

                'R' => curr_piece = Rc::new(Rook{player: Player::White}),
                'N' => curr_piece = Rc::new(Knight{player: Player::White}),
                'B' => curr_piece = Rc::new(Bishop{player: Player::White}),
                'Q' => curr_piece = Rc::new(Queen{player: Player::White}),
                'K' => curr_piece = Rc::new(King{player: Player::White}),
                'P' => curr_piece = Rc::new(Pawn{player: Player::White}),

                '/' => continue, // we don't want it to increment the curr indx
                _ => println!("unknown piece")
            }
            if *curr_piece.get_player() == Player::White {
                white_pieces.push(curr_piece.clone());
                board.board[curr_idx] = curr_piece.clone();
            } else if *curr_piece.get_player() == Player::Black {
                black_pieces.push(curr_piece.clone());
                board.board[curr_idx] = curr_piece.clone();
            }
            curr_idx += 1;
        }
        return (white_pieces, black_pieces)
    }
}