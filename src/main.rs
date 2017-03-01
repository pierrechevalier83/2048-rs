extern crate rand;

mod board;
mod algorithm;
use rand::{thread_rng, Rng};

enum Direction {
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
    fn data(&self) -> [i32; 16] {
        self.data
    }
    fn horizontal(&mut self, dir: Direction) {
        self.data
            .chunks_mut(4)
            .map(|mut row| {
                let after = match dir {
                    Direction::right => algorithm::slide_right(&row),
                    Direction::left => algorithm::slide_left(&row),
                    _ => algorithm::slide_right(&row),
                };
                for i in 0..4 {
                    row[i] = after[i];
                }
            })
            .collect::<Vec<_>>();
    }
    fn vertical(&mut self, dir: Direction) {
        algorithm::transpose(&mut self.data);
        match dir {
            Direction::up => self.left(),
            Direction::down => self.right(),
            _ => (),
        };
        algorithm::transpose(&mut self.data);
    }
    fn right(&mut self) {
        self.horizontal(Direction::right);
    }
    fn left(&mut self) {
        self.horizontal(Direction::left);
    }
    fn up(&mut self) {
        self.vertical(Direction::up);
    }
    fn down(&mut self) {
        self.vertical(Direction::down);
    }
}

fn main() {
    let board = board::Board::new();
    let mut game = Game::new();
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
