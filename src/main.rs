#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod algorithm;
mod board;
mod display;
mod game;

use std::io::{stdout, BufWriter, Write};
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::QueueableCommand;
use game::GameStatus;

fn main() -> Result<(), std::io::Error> {
    let stdout_raw = stdout();
    let mut stdout = BufWriter::new(stdout_raw.lock());
    crossterm::terminal::enable_raw_mode()?;

    let board = board::Board::new();
    let mut game = game::Game::new();

    display::display_game(&mut stdout, &board, &game)?.flush()?;

    loop {
        if crossterm::event::poll(Duration::from_millis(50))? {
            let mut changed = false;
            match crossterm::event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => match game.status() {
                    GameStatus::Interrupted | GameStatus::Won | GameStatus::Lost => break,
                    _ => {
                        game.interrupt();
                    }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('y'),
                    ..
                }) => match game.status() {
                    GameStatus::Interrupted | GameStatus::Won => break,
                    _ => (),
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('n'),
                    ..
                }) => match game.status() {
                    GameStatus::Interrupted | GameStatus::Won => game.go_on(),
                    _ => (),
                },
                Event::Key(KeyEvent { code, .. }) => {
                    if let GameStatus::Ongoing = game.status() {
                        changed = game.movement(code)
                    }
                }
                _ => (),
            };
            if changed {
                game.new_tile();
            } else {
                game.check_if_lost();
            }
            display::display_game(&mut stdout, &board, &game)?;
        };
        if game.status() == &game::GameStatus::Won {
            display::display_game(&mut stdout, &board, &game)?;
        }
        stdout.flush()?;
    }
    stdout.queue(crossterm::cursor::Show)?.flush()?;
    crossterm::terminal::disable_raw_mode()
}
