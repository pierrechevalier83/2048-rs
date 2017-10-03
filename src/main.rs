#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
extern crate rand;
extern crate termion;

mod algorithm;
mod board;
mod display;
mod game;

use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout};
use std::thread;

use game::GameStatus;

fn main() {
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let board = board::Board::new();
    let mut game = game::Game::new();

    display::display_game(&mut stdout, &board, &game);
    stdout.flush().unwrap();

    loop {
        if let Some(evt) = stdin.next() {
            let mut changed = false;
            match evt.unwrap() {
                Event::Key(Key::Char('q')) => {
                    match game.status() {
                        GameStatus::Interrupted | GameStatus::Won | GameStatus::Lost => break,
                        _ => {
                            game.interrupt();
                        }
                    }
                }
                Event::Key(Key::Char('y')) => {
                    match game.status() {
                        GameStatus::Interrupted | GameStatus::Won => break,
                        _ => (),
                    }
                }
                Event::Key(Key::Char('n')) => {
                    match game.status() {
                        GameStatus::Interrupted | GameStatus::Won => game.go_on(),
                        _ => (),
                    }
                }
                Event::Key(Key::Up) => {
                    if let GameStatus::Ongoing = game.status() {
                        changed = game.up();
                    }
                }
                Event::Key(Key::Down) => {
                    if let GameStatus::Ongoing = game.status() {
                        changed = game.down();
                    }
                }
                Event::Key(Key::Left) => {
                    if let GameStatus::Ongoing = game.status() {
                        changed = game.left();
                    }
                }
                Event::Key(Key::Right) => {
                    if let GameStatus::Ongoing = game.status() {
                        changed = game.right();
                    }
                }
                _ => (),
            };
            if changed {
                game.new_tile();
            } else {
                game.check_if_lost();
            }
            display::display_game(&mut stdout, &board, &game);
        };
        if game.status() == game::GameStatus::Won {
            display::display_game(&mut stdout, &board, &game);
            thread::sleep(std::time::Duration::from_millis(150));
		} else {  
            thread::sleep(std::time::Duration::from_millis(50));
        }
		stdout.flush().unwrap();
    }
}
