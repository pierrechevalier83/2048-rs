extern crate rand;
extern crate termion;

mod algorithm;
mod board;
mod game;

use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn header<W>(out: &mut W) where W: Write {
    write!(out, "2048-rs [pierrec.tech]\r\n").unwrap();
}

fn footer<W>(out: &mut W) where W: Write {
    write!(out, "    [ ← ↑ → ↓ ], q for quit\r\n").unwrap();
}

fn clear<W>(out: &mut W) where W: Write {
    write!(out,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1)).unwrap();
}

fn display_game<W>(out: &mut W, board: &board::Board, game: &game::Game) where W: Write {
    clear(out);
    header(out);
    board.print(game.data(), out);
    footer(out);
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let board = board::Board::new();
    let mut game = game::Game::new();

    display_game(&mut stdout, &board, &game);
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
        display_game(&mut stdout, &board, &game);
        stdout.flush().unwrap();
    }

}
