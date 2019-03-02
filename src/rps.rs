mod input;

use input::*;
use core::cmp::Ordering;

#[derive(PartialEq)]
enum RPSMove {
    Rock,
    Paper,
    Scissors
}

impl PartialOrd for RPSMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => Some(Ordering::Less),
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

use RPSMove::*;

struct RPS;

trait RPSMoves {
    fn to_rps(&self) -> Option<RPSMove>;
}

impl RPSMoves for String {
    fn to_rps(&self) -> Option<RPSMove> {
        match self.as_ref() {
            "r" | "R" => Some(Rock),
            "p" | "P" => Some(Paper),
            "s" | "S" => Some(Scissors),
            _ => None
        }
    }
}

impl RPS {
    fn run() {
        loop {
            let move_one = Self::player_move("Player One");
            let move_two = Self::player_move("Player Two");

            if move_one == move_two {
                println!("Draw")
            } else if move_one > move_two {
                println!("Player One wins!");
            } else {
                println!("Player Two wins!");
            }

            if !Self::prompt_new_game() {
                println!("GG");
                break;
            }
        }
    }

    fn player_move(name: &str) -> RPSMove  {
        loop {
            let prompt = format!("{} [r/p/s]: ", name);
            if let Some(player_move) = input(&prompt).to_rps() {
                break player_move
            } else {
                println!("Invalid move");
            }
        }
    }

    fn prompt_new_game() -> bool {
        match input_yes_no("Another game? [y/n]: ") {
            Ok(ans) => ans,
            Err(msg) => {
                println!("{}", msg);
                Self::prompt_new_game()
            }
        }
    }
}

fn main() {
    RPS::run();
}
