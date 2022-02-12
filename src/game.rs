use super::algorithm;
use crossterm::event::KeyCode;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_xoshiro::Xoshiro256Plus;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub enum GameStatus {
    Ongoing,
    Won,
    Lost,
    Interrupted,
}

#[derive(Clone)]
pub struct Game {
    status: GameStatus,
    already_won: bool,
    score: i32,
    data: [i32; 16],
    rng: Xoshiro256Plus,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = Xoshiro256Plus::from_entropy();
        let mut data = [0; 16];
        data[0] = 1;
        data[1] = 1;
        data.shuffle(&mut rng);
        Game {
            status: GameStatus::Ongoing,
            already_won: false,
            score: 0,
            data,
            rng,
        }
    }
    pub fn data(&self) -> [i32; 16] {
        self.data
    }
    pub fn score(&self) -> i32 {
        self.score
    }
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    pub fn interrupt(&mut self) {
        self.status = GameStatus::Interrupted;
    }
    pub fn go_on(&mut self) {
        self.status = GameStatus::Ongoing;
    }
    pub fn check_if_lost(&mut self) {
        let mut copy = self.clone();
        if !(copy.right() || copy.left() || copy.up() || copy.down()) {
            self.status = GameStatus::Lost;
        }
    }
    fn horizontal(&mut self, dir: Direction) -> bool {
        let mut mutated = false;
        let mut score = 0;
        let mut won = false;
        self.data.chunks_mut(4).for_each(|row| {
            let (new_row, new_score) = match dir {
                Direction::Right => algorithm::slide_right(row),
                Direction::Left => algorithm::slide_left(row),
                _ => (row.to_vec(), 0),
            };
            if new_score == 2048 {
                won = true;
            }
            score += new_score;
            for i in 0..4 {
                if row[i] != new_row[i] {
                    row[i] = new_row[i];
                    mutated = true;
                }
            }
        });
        self.score += score;
        if won && !self.already_won {
            self.status = GameStatus::Won;
            self.already_won = true;
        }
        mutated
    }
    fn vertical(&mut self, dir: Direction) -> bool {
        algorithm::transpose(&mut self.data);
        let mutated = match dir {
            Direction::Up => self.left(),
            Direction::Down => self.right(),
            _ => false,
        };
        algorithm::transpose(&mut self.data);
        mutated
    }
    pub fn new_tile(&mut self) {
        let value = if self.rng.gen::<i32>() % 10 == 1 {
            2
        } else {
            1
        };

        self.data[self
            .data
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x == 0)
            .map(|(i, _)| i)
            .choose(&mut self.rng)
            .unwrap()] = value;
    }

    pub fn right(&mut self) -> bool {
        self.horizontal(Direction::Right)
    }
    pub fn left(&mut self) -> bool {
        self.horizontal(Direction::Left)
    }
    pub fn up(&mut self) -> bool {
        self.vertical(Direction::Up)
    }
    pub fn down(&mut self) -> bool {
        self.vertical(Direction::Down)
    }

    pub fn movement(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up | KeyCode::Char('k') => self.up(),
            KeyCode::Left | KeyCode::Char('h') => self.left(),
            KeyCode::Right | KeyCode::Char('l') => self.right(),
            KeyCode::Down | KeyCode::Char('j') => self.down(),
            _ => false,
        }
    }
}
