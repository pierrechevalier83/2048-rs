use termion;
use board;
use game;
use std::io::Write;

fn header<W>(out: &mut W)
    where W: Write
{
    write!(out, "2048-rs [pierrec.tech]\r\n").unwrap();
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
    header(out);
    board.print(game.data(), out);
    footer(out);
}
