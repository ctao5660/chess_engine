use std::io::{stdin, stdout, Write};
use crate::board::Board;
use crate::game::Game;

pub struct Play<'a> {
    pub(crate) game: &'a Game,
}