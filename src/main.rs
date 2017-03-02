extern crate rand;
extern crate termion;

mod algorithm;
mod board;
mod display;
mod game;

use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let board = board::Board::new();
    let mut game = game::Game::new();

    display::display_game(&mut stdout, &board, &game);
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => game.up(),
            Event::Key(Key::Down) => game.down(),
            Event::Key(Key::Left) => game.left(),
            Event::Key(Key::Right) => game.right(),
            _ => (),
        };
        game.new_tile();
        display::display_game(&mut stdout, &board, &game);
        stdout.flush().unwrap();
    }

}
