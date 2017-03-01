extern crate rand;

mod algorithm;
mod board;
mod game;

fn main() {
    let board = board::Board::new();
    let mut game = game::Game::new();
    board.print(game.data());
    game.right();
    board.print(game.data());
    game.left();
    board.print(game.data());
    game.up();
    board.print(game.data());
    game.down();
    board.print(game.data());
}
