use crate::game::Game;

mod board;
mod game;
mod play;


fn main() {
    let mut game = Game::new();
    game.start_game();


}

mod pieces {
    pub mod pieces;
    pub mod king;
    pub mod rook;
    pub mod knight;
    pub mod pawn;
    pub mod queen;
    pub mod bishop;
}

