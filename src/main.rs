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
                        game::GameStatus::Interrupted => break,
                        game::GameStatus::Won => break,
                        game::GameStatus::Lost => break,
                        _ => {
                            game.interrupt();
                        }
                    }
                }
                Event::Key(Key::Char('y')) => {
                    match game.status() {
                        game::GameStatus::Interrupted => break,
                        game::GameStatus::Won => break,
                        _ => (),
                    }
                }
                Event::Key(Key::Char('n')) => {
                    match game.status() {
                        game::GameStatus::Interrupted => game.go_on(),
                        game::GameStatus::Won => game.go_on(),
                        _ => (),
                    }
                }
                Event::Key(Key::Up) => {
                    match game.status() {
                        game::GameStatus::Ongoing => {
                            changed = game.up();
                        }
                        _ => (),
                    }
                }
                Event::Key(Key::Down) => {
                    match game.status() {
                        game::GameStatus::Ongoing => {
                            changed = game.down();
                        }
                        _ => (),
                    }
                }
                Event::Key(Key::Left) => {
                    match game.status() {
                        game::GameStatus::Ongoing => {
                            changed = game.left();
                        }
                        _ => (),
                    }
                }
                Event::Key(Key::Right) => {
                    match game.status() {
                        game::GameStatus::Ongoing => {
                            changed = game.right();
                        }
                        _ => (),
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
            std::thread::sleep(std::time::Duration::from_millis(150));
		} else {  
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
		stdout.flush().unwrap();
    }
}
