use crate::board::{Board};
use std::collections::{HashMap};
use std::rc::Rc;
use regex::Regex;
use lazy_static::lazy_static;
use crate::pieces::pieces::{Piece};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Player {
    White,
    Black,
    Empty,
}
pub struct Game {
    pub board: Board,
    active_color: Player, // who's turn
    castling_rights: HashMap<Player, [bool; 2]>, // [bool, bool] = [queenSide, kingSide]
    pieces: HashMap<Player, Vec<Rc<dyn Piece>>>,
    en_passant_target: usize, // hashset by index into board arr
    halfmove_clock: u16,
    fullmove_clock: i32,

}
lazy_static! {
    static ref FEN_REGEX: Regex = Regex::new(r"\s*([rnbqkpRNBQKP1-8]+\/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*").unwrap();
}

impl Game {

    // Get player - select piece - select valid move from that piece (todo: filter valid moves based on pins/check
    // Implement check logic  - if player is in check they have to move king or block with piece
    // implement en passant - add valid moves to

    pub fn start_game(& mut self){
        self.print_board();
        let mut input = String::new();
        println!("enter piece coordinate");
        std::io::stdin().read_line(&mut input).unwrap();
        let prev_coord = Board::coordinate_to_idx(&input.trim());
        let piece = self.board.board[prev_coord].clone();
        let grid_moves = piece.get_moves(&self.board, prev_coord as i32);
        let coord_moves: Vec<String> = grid_moves.iter().map(|x| Board::grid_to_coordinate(*x)).collect();
        let mut piece_str = String::new();
        println!("piece {} moves {:?}. Pick one", piece.get_name(), coord_moves);
        std::io::stdin().read_line(&mut piece_str).unwrap();
        let coord = Board::coordinate_to_idx(&piece_str.trim());
        self.board.place_piece(coord, prev_coord,&piece);
        self.print_board();

    }
    pub fn validate_fen(fen: &str) -> bool {
        FEN_REGEX.is_match(fen)
    }
    pub fn opposite_player(p1:&Player, p2:&Player) -> bool {
        if *p1 == Player::White {
            return *p2 == Player::Black;
        } else if *p1 == Player::Black {
            return *p2 == Player::White;
        } else {
            return false
        }
    }
    pub fn new() -> Game{
        let game = Game::game_state_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        return game
    }
    pub fn print_board(&self) {
        Board::print_board(&self.board.board);
    }
    pub(crate) fn game_state_from_fen(fen: &str) -> Game{
        let mut board: Board = Board{ board: Vec::new() };
        let mut active_color: Player = Player::Empty;
        let mut castling_rights: HashMap<Player, [bool; 2]> = HashMap::new();
        let mut enpassant_pieces: usize = 65;
        let mut enpassant_pieces: usize = 65;
        let mut halfmove_clock: u16 = 0;
        let mut fullmove_clock: i32 = 1;
        let mut pieces: HashMap<Player, Vec<Rc<dyn Piece>>> = HashMap::new();
        pieces.insert(Player::White, Vec::new());
        pieces.insert(Player::Black, Vec::new());
        if !Game::validate_fen(fen) {
            panic!("invalid fen")
        }
        let parts = fen.split(" ");
        for (i, part) in parts.enumerate() {
            match i {
                0 => {
                    let board_ret = Board::board_from_fen( part);
                    board = board_ret.0;
                    pieces.insert(Player::White, board_ret.1);
                    pieces.insert(Player::Black, board_ret.2);
                },
                1 => active_color = Game::parse_active_color(part),
                2 => castling_rights = Game::parse_castling_rights(part),
                3 => enpassant_pieces = Game::parse_enpassant_pieces(part),
                4 => halfmove_clock = Game::parse_halfmove_clock(part),
                5 => fullmove_clock = Game::parse_fullmove_clock(part),
                _ => println!("{}", i)
            }
        }

        return Game{
            board,
            active_color,
            castling_rights,
            pieces,
            en_passant_target: 65,
            halfmove_clock,
            fullmove_clock,
        }
    }
    fn parse_active_color(color: &str) -> Player{
        match color {
            "b" => return Player::Black,
            "w" => return Player::White,
            _ => panic!("invalid active color in PGN")
        }
    }
    fn parse_castling_rights(rights_str: &str) -> HashMap<Player, [bool; 2]>{
        let mut black_castle = [false, false];
        let mut white_castle = [false, false];
        let mut castling_rights = HashMap::new();
        if rights_str == "-" {
            castling_rights.insert(Player::White, white_castle);
            castling_rights.insert(Player::Black, black_castle);
            return castling_rights;
        }
        for c in rights_str.chars() {
            match c {
                'q' => black_castle[0] = true,
                'k' => black_castle[1] = true,
                'Q' => white_castle[0] = true,
                'K' => white_castle[1] = true,
                _ => panic!("invalid castling rights string")
            }
        }
        castling_rights.insert(Player::White, white_castle);
        castling_rights.insert(Player::Black, black_castle);
        return castling_rights;
    }
    fn parse_enpassant_pieces(enpassant_str: &str) -> usize {
        match enpassant_str {
            "-" => 65,
            _ => return Board::coordinate_to_idx(enpassant_str),
        }
    }
    fn parse_halfmove_clock(halfmove_str: &str ) -> u16{
        return halfmove_str.parse::<u16>().unwrap();
    }
    fn parse_fullmove_clock(fullmove_str: &str) -> i32{
        return fullmove_str.parse::<i32>().unwrap();
    }
}