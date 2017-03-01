extern crate rand;

mod board;
use rand::{thread_rng, Rng};

#[cfg(test)]
mod slide_right_test {
    use super::slide_right;
    use super::slide_left;
    #[test]
    fn test_slide_right_with_one_element() {
        assert_eq!(vec![0, 0, 0, 1], slide_right([0, 1, 0, 0]));
    }
    #[test]
    fn test_slide_left_with_one_element() {
        assert_eq!(vec![1, 0, 0, 0], slide_left([0, 1, 0, 0]));
    }
    #[test]
    fn test_slide_right_with_two_different_elements() {
        assert_eq!(vec![0, 0, 1, 2], slide_right([1, 0, 2, 0]));
    }
    #[test]
    fn test_slide_left_with_two_different_elements() {
        assert_eq!(vec![1, 2, 0, 0], slide_left([1, 0, 2, 0]));
    }
    #[test]
    fn test_slide_right_with_two_same_elements() {
        assert_eq!(vec![0, 0, 0, 2], slide_right([1, 0, 1, 0]));
    }
    #[test]
    fn test_slide_left_with_two_same_elements() {
        assert_eq!(vec![2, 0, 0, 0], slide_left([1, 0, 1, 0]));
    }
    #[test]
    fn test_slide_right_with_three_same_elements() {
        assert_eq!(vec![0, 0, 1, 2], slide_right([1, 0, 1, 1]));
    }
    #[test]
    fn test_slide_left_with_three_same_elements() {
        assert_eq!(vec![2, 1, 0, 0], slide_left([1, 0, 1, 1]));
    }
    #[test]
    fn test_slide_right_with_three_different_elements() {
        assert_eq!(vec![0, 0, 2, 2], slide_right([1, 0, 1, 2]));
        assert_eq!(vec![0, 2, 1, 2], slide_right([2, 0, 1, 2]));
        assert_eq!(vec![0, 0, 2, 2], slide_right([0, 1, 1, 2]));
    }
    #[test]
    fn test_slide_left_with_three_different_elements() {
        assert_eq!(vec![2, 2, 0, 0], slide_left([1, 0, 1, 2]));
        assert_eq!(vec![2, 1, 2, 0], slide_left([2, 0, 1, 2]));
        assert_eq!(vec![2, 2, 0, 0], slide_left([0, 1, 1, 2]));
    }
    #[test]
    fn test_slide_right_with_four_same_elements() {
        assert_eq!(vec![0, 0, 2, 2], slide_right([1, 1, 1, 1]));
    }
    #[test]
    fn test_slide_left_with_four_same_elements() {
        assert_eq!(vec![2, 2, 0, 0], slide_left([1, 1, 1, 1]));
    }
    #[test]
    fn test_slide_right_with_four_different_elements() {
        assert_eq!(vec![1, 2, 1, 2], slide_right([1, 2, 1, 2]));
    }
    #[test]
    fn test_slide_left_with_four_different_elements() {
        assert_eq!(vec![1, 2, 1, 2], slide_left([1, 2, 1, 2]));
    }
}

fn merge_backward(slice: &mut [i32]) {
    if slice[0] == slice[1] && slice[1] != 0 {
        slice[0] = 0;
        slice[1] += 1;
    }
}

fn stable_partition<T, I, F>(slice: I, pred: F) -> Vec<T>
    where T: Copy,
          I: IntoIterator<Item = T>,
          for<'r> F: Fn(&'r T) -> bool
{
    let (mut left, right): (Vec<T>, Vec<T>) = slice.into_iter().partition(pred);
    left.extend(right.iter());
    left
}

fn slide_right(data: [i32; 4]) -> Vec<i32> {
    let mut ret = stable_partition(data.iter().cloned(), |x| *x == 0);
    let mut index = data.len();
    while index > 1 {
        merge_backward(&mut ret[index - 2..index]);
        index -= 1;
    }
    stable_partition(ret.iter().cloned(), |x| *x == 0)
}

fn slide_left(data: [i32; 4]) -> Vec<i32> {
    let mut ret = data.clone();
    ret.reverse();
    slide_right(ret).iter().cloned().rev().collect::<Vec<_>>()
}

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
