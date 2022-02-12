use super::board;
use super::game;
use crossterm::terminal::ClearType;
use crossterm::QueueableCommand;
use game::GameStatus;
use std::io::Write;

fn header<W>(out: &mut W, score: i32) -> Result<(), std::io::Error>
where
    W: Write,
{
    write!(
        out,
        "2048-rs [pierrec.tech]{num:>pad$}\r\n",
        num = score,
        pad = 11
    )
}

fn footer<W>(out: &mut W, status: &GameStatus) -> Result<(), std::io::Error>
where
    W: Write,
{
    write!(
        out,
        "{}",
        match status {
            GameStatus::Ongoing => "    [ ← ↑ → ↓ ], q for quit\r\n",
            GameStatus::Lost => "    [  🎮 ⛔  ], q for quit\r\n",
            GameStatus::Interrupted => "    [  🎮 🚦  ], quit? (y/n)\r\n",
            GameStatus::Won => "    [ 🎉🎉🎉 ], quit? (y/n)\r\n",
        }
    )
}

fn clear<W>(out: &mut W) -> Result<&mut W, std::io::Error>
where
    W: Write,
{
    out.queue(crossterm::terminal::Clear(ClearType::All))?
        .queue(crossterm::cursor::Hide)?
        .queue(crossterm::cursor::MoveTo(1, 1))
}

pub fn display_game<'a, W>(
    out: &'a mut W,
    board: &board::Board,
    game: &game::Game,
) -> Result<&'a mut W, std::io::Error>
where
    W: Write,
{
    header(clear(out)?, game.score())?;
    let status = game.status();
    match status {
        GameStatus::Ongoing => board.print(game.data(), out),
        GameStatus::Lost => board.print_lost(game.data(), out),
        GameStatus::Interrupted => board.print_inactive(game.data(), out),
        GameStatus::Won => board.print_won(game.data(), out),
    };
    footer(out, status)?;
    Ok(out)
}
