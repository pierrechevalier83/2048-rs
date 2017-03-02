extern crate rand;

use algorithm;
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

pub struct Game {
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
    pub fn data(&self) -> [i32; 16] {
        self.data
    }
    fn horizontal(&mut self, dir: Direction) -> bool {
        let mut mutated = false;
        let mut score = 0;
        self.data
            .chunks_mut(4)
            .map(|mut row| {
                let after = match dir {
                    Direction::right => algorithm::slide_right(&row),
                    Direction::left => algorithm::slide_left(&row),
                    _ => row.iter().cloned().collect::<Vec<_>>(),
                };
                for i in 0..4 {
                    if row[i] != after[i] {
                        row[i] = after[i];
                        score += row[i];
                        mutated = true;
                    }
                }
            })
            .collect::<Vec<_>>();
        self.score += score;
        mutated
    }
    fn vertical(&mut self, dir: Direction) -> bool {
        algorithm::transpose(&mut self.data);
        let mutated = match dir {
            Direction::up => self.left(),
            Direction::down => self.right(),
            _ => false,
        };
        algorithm::transpose(&mut self.data);
        mutated
    }
    pub fn new_tile(&mut self) {
        let mut value = 1;
        if rand::random::<i32>() % 10 == 1 {
            value = 2;
        }
        let zeroes_index = self.data
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x == 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        self.data[zeroes_index[rand::random::<usize>() % zeroes_index.len()]] = value;
    }
    pub fn right(&mut self) -> bool {
        self.horizontal(Direction::right)
    }
    pub fn left(&mut self) -> bool {
        self.horizontal(Direction::left)
    }
    pub fn up(&mut self) -> bool {
        self.vertical(Direction::up)
    }
    pub fn down(&mut self) -> bool {
        self.vertical(Direction::down)
    }
}
