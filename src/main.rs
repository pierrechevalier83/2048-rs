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
use game::GameStatus;

fn exit_prompt() -> bool {
    let mut exit = false;
    for c in stdin().events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('y')) => {
                exit = true;
                break;
            }
            Event::Key(Key::Char('n')) => break,
            Event::Key(Key::Char('q')) => break,
            _ => (),
        };
    }
    exit
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let board = board::Board::new();
    let mut game = game::Game::new();

    display::display_game(&mut stdout, &board, &game);
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        let changed = match evt {
            Event::Key(Key::Char('q')) => {
                game.interrupt();
                display::display_game(&mut stdout, &board, &game);
                if exit_prompt() {
                    break;
                } else {
                    game.go_on();
                }
                false
            }
            Event::Key(Key::Up) => game.up(),
            Event::Key(Key::Down) => game.down(),
            Event::Key(Key::Left) => game.left(),
            Event::Key(Key::Right) => game.right(),
            _ => false,
        };
        if changed {
            game.new_tile();
        } else {
            game.check_if_lost();
        }
        display::display_game(&mut stdout, &board, &game);
        match game.status() {
            GameStatus::won => {
                if exit_prompt() {
                    break;
                }
            }
            _ => (),
        };
        stdout.flush().unwrap();
    }

    stdout.flush().unwrap();
}
