use std::{fmt, io, process};
use std::convert::TryFrom;
use std::fmt::Display;
use crate::tic_tac_toe::Index::{One, Two, Zero};
use crate::tic_tac_toe::Player::{O, X};
use crate::tic_tac_toe::TicTacToeEvent::{Exited, InputIgnored, Moved, MovedAndTie, MovedAndWon};
use crate::tic_tac_toe::TicTacToeInput::{Exit, Move, Unknown};
use crate::types::Application;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Player {
    #[default]
    X,
    O,
}
pub type Board = [[Option<Player>; 3]; 3];

#[derive(Default)]
pub struct TicTacToeState {
    board: Board,
    turn: Player,
}

#[derive(Clone, Copy)]
pub enum TicTacToeInput {
    Move(Index, Index),
    Exit,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub enum TicTacToeEvent {
    Moved(Player, usize, usize),
    Exited,
    MovedAndWon(Player, usize, usize),
    MovedAndTie(Player, usize, usize),
    InputIgnored,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Index {
    Zero = 0,
    One = 1,
    Two = 2,
}

impl TryFrom<char> for Index {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            _ => Err("Illegal index, expected one of 0, 1 or 2"),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X => write!(f, "X"),
            O => write!(f, "0"),
        }
    }
}

fn to_input((c0, c1): (char, char)) -> TicTacToeInput {
    match (c0, c1) {
        ('x', _) => {
            Exit
        }
        ('0'..='2', '0'..='2') => {
            Move(Index::try_from(c0).unwrap(), Index::try_from(c1).unwrap())
        }
        _ => {
            Unknown
        }
    }
}

fn is_winner(board: Board, p: Player) -> bool {
    for i in 0..2 {
        if (Some(p), Some(p), Some(p)) ==
            (board[i][0], board[i][1], board[i][2]) {
            return true;
        }
    }

    for j in 0..2 {
        if (Some(p), Some(p), Some(p)) ==
            (board[0][j], board[1][j], board[2][j]) {
            return true;
        }
    }

    if (Some(p), Some(p), Some(p)) ==
        (board[0][0], board[1][1], board[2][2]) ||
        (Some(p), Some(p), Some(p)) ==
            (board[0][2], board[1][1], board[2][0]) {
        true
    } else {
        false
    }
}

fn is_tie(board: Board) -> bool {
    if !board[0].iter().any(|&p| p == None) &&
        !board[1].iter().any(|&p| p == None) &&
        !board[2].iter().any(|&p| p == None) {
        return true;
    }
    false
}

fn update(state_and_input: (&mut TicTacToeState, TicTacToeInput)) -> TicTacToeEvent {
    match state_and_input {
        (state, Move(r, c)) if state.board[r as usize][c as usize] != None => {
            println!("Row {}, column {} not available", r as usize, c as usize);
            InputIgnored
        }

        (state, Move(r, c)) => {
            let p = state.turn;
            state.board[r as usize][c as usize] = Some(p);
            match p {
                X => state.turn = O,
                O => state.turn = X,
            }

            if is_winner(state.board, p) {
                MovedAndWon(p, r as usize, c as usize)
            } else if is_tie(state.board) {
                MovedAndTie(p, r as usize, c as usize)
            } else {
                Moved(p, r as usize, c as usize)
            }
        }
        (_, Exit) => {
            Exited
        }
        (_, Unknown) => {
            InputIgnored
        }
    }
}


fn process_event(event: TicTacToeEvent) {
    println!("{:?}", event);

    match event {
        Exited => {
            process::exit(0x0100);
        }
        MovedAndWon(player, _, _) => {
            println!("Player {} wins", player);
            process::exit(0x0200);
        }
        MovedAndTie(_, _, _) => {
            println!("Tie, nobody wins");
            process::exit(0x0200);
        }

        _ => {}
    }
}

impl TicTacToeState {
    pub fn init() -> TicTacToeState {
        TicTacToeState::default()
    }
}

impl Display for TicTacToeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row_idx, row) in self.board.iter().enumerate() {
            write!(f, "{} ", row_idx)?;
            write!(f, "     ")?;
            for (cell_idx, cell) in row.iter().enumerate() {
                if cell_idx > 0 {
                    write!(f, " | ")?
                }
                match cell {
                    Some(player) => write!(f, "{}", player)?,
                    None => write!(f, " ")?,
                };
            }
            if row_idx < 2 {
                writeln!(f)?;
                write!(f, "      ---+---+---")?;
                writeln!(f)?;
            }
        }
        writeln!(f)?;
        writeln!(f)?;
        write!(f, "       0   1   2")?;
        writeln!(f)?;
        Ok(())
    }
}

fn listener() -> (char, char) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut f = input.trim().chars().filter(|c| c != &' ' && c != &',');
    let c0 = f.nth(0).unwrap();
    let c1 = match f.nth(0) {
        Some(c) => c,
        None => ' '
    };
    (c0, c1)
}

pub type TicTacToeGame = Application<(char, char), TicTacToeState, TicTacToeInput, TicTacToeEvent>;

impl TicTacToeGame {
    pub(crate) fn new() -> Application<(char, char), TicTacToeInput, TicTacToeState, TicTacToeEvent> {
        Application {
            state: TicTacToeState::init(),
            interaction_listener: &listener,
            interaction_handler: &to_input,
            domain_logic: &update,
            publish_state: &publish,
            event_handler: &process_event,
            respond: &display_message,
        }
    }
}

fn display_message(m: &dyn Display) {
    println!("{}", m);
}

#[allow(unused_variables)]
fn publish(state: &TicTacToeState) {}

pub fn main() {
    TicTacToeGame::new().run();
}
