mod board;

fn main() {
    let board = board::Board::new();
    board.print((0..16));
}
