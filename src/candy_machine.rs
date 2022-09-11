use std::{fmt, io, process};
use std::fmt::{Debug, Display};
use crate::candy_machine::CandyMachineEvent::{CandyReleased, CoinReceived, Exited, InputIgnored};
use crate::candy_machine::CandyMachineInput::{Coin, Exit, Turn, Unknown};
use crate::application::Application;

#[derive(Debug)]
pub struct CandyMachineState {
    locked: bool,
    candies: usize,
    coins: usize,
}

impl fmt::Display for CandyMachineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Locked: {},  Coins: {}, Candies: {}", self.locked, self.coins, self.candies)
    }
}

pub enum CandyMachineInput {
    Coin,
    Turn,
    Exit,
    Unknown
}

#[derive(Debug, PartialEq)]
pub enum CandyMachineEvent {
    CandyReleased,
    CoinReceived,
    Exited,
    InputIgnored
}

fn listener() -> char {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.chars().nth(0).unwrap()
}

fn to_input(c: char) -> CandyMachineInput {
    match c {
        'c' => Coin,
        't' => Turn,
        'x' => Exit,
        _ => Unknown
    }
}

fn process_event(event: CandyMachineEvent) {
    println!("{:?}", event);
    if event == Exited {
        process::exit(0x0100);
    }
}

fn display_message(m: &dyn Display) {
    println!("{}", m);
}

fn update(state_and_input: (&mut CandyMachineState, CandyMachineInput)) -> CandyMachineEvent {
    match state_and_input {
        (state, Coin) if state.candies > 0 && state.locked == false => {
            state.coins += 1;
            state.locked = true;
            CoinReceived
        }
        (state, Turn) if state.coins > 0 && state.locked == true => {
            {
                state.candies -= 1;
                state.locked = false;
                CandyReleased
            }
        }
        (_, Exit) => {
            Exited
        }
        _ => {
            InputIgnored
        }
    }
}

pub type CandyMachine = Application<char, CandyMachineState, CandyMachineInput, CandyMachineEvent>;

impl  CandyMachine {
    pub(crate) fn new() -> Application<char, CandyMachineInput, CandyMachineState, CandyMachineEvent> {
        Application {
            state: CandyMachineState {locked: false, candies: 10, coins: 0},
            interaction_listener: &listener,
            interaction_handler: &to_input,
            domain_logic: &update,
            publish_state: &publish,
            event_handler: &process_event,
            respond: &display_message
        }
    }
}

#[allow(unused_variables)]
fn publish(state: &CandyMachineState)  {
}

pub(crate) fn main() {
    CandyMachine::new().run();
}

