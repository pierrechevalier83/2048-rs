use termion;
use board;
use game;
use std::io::Write;
use game::GameStatus;

fn header<W>(out: &mut W, score: i32)
    where W: Write
{
    write!(out,
           "2048-rs [pierrec.tech]{num:>pad$}\r\n",
           num = score,
           pad = 11)
        .unwrap();
}

fn footer<W>(out: &mut W, status: GameStatus)
    where W: Write
{
    let text = match status {
        GameStatus::Ongoing => "    [ ← ↑ → ↓ ], q for quit\r\n",
        GameStatus::Lost => "    [  🎮 ⛔  ], q for quit\r\n",
        GameStatus::Interrupted => "    [  🎮 🚦  ], quit? (y/n)\r\n",
        GameStatus::Won => "    [ 🎉🎉🎉 ], quit? (y/n)\r\n",
    };
    write!(out, "{}", text).unwrap();
}

fn clear<W>(out: &mut W)
    where W: Write
{
    write!(out,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1))
        .unwrap();
}

pub fn display_game<W>(out: &mut W, board: &board::Board, game: &game::Game)
    where W: Write
{
    clear(out);
    header(out, game.score());
	if game.status() == GameStatus::Ongoing {
        board.print(game.data(), out);
	} else {
        board.print_inactive(game.data(), out);
	}
    footer(out, game.status());
}
