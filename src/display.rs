use termion;
use board;
use game;
use std::io::Write;

fn header<W>(out: &mut W, score: i32)
    where W: Write
{
    write!(out,
           "2048-rs [pierrec.tech]{num:>pad$}\r\n",
           num = score,
           pad = 11)
        .unwrap();
}

fn footer<W>(out: &mut W)
    where W: Write
{
    write!(out, "    [ ← ↑ → ↓ ], q for quit\r\n").unwrap();
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
    board.print(game.data(), out);
    footer(out);
}
