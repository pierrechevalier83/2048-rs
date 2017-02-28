extern crate rand;

mod board;
use rand::{thread_rng, Rng};

enum Move {
    up,
    down,
    left,
    right,
}

enum GameStatus {
    ongoing,
    won,
    lost,
    interrupted,
}

struct Game {
    status: GameStatus,
    score: i32,
    data: [i32; 16],
}

impl Game {
    pub fn new() -> Game {
        let mut rng = thread_rng();
        let mut data = [0; 16];
        data[0] = 1;
        data[1] = 1;
        rng.shuffle(&mut data);
        Game {
            status: GameStatus::ongoing,
            score: 0,
            data: data,
        }
    }
    fn data(self) -> [i32; 16] {
        self.data
    }
}

fn main() {
    let board = board::Board::new();
    let game = Game::new();
    board.print(game.data());
}
