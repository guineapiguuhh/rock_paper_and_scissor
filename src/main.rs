use owo_colors::OwoColorize;
use rand::{Rng, rngs::ThreadRng};

pub mod player;
pub mod style;
pub mod io_macros;

use crate::player::*;

const WIN_TEXT: &'static str = " is the winner!";

fn main() {
    match enable_ansi_support::enable_ansi_support() {
        Ok(_) => {},
        Err(_) => println!("failed to enable ansi!"),
    }

    let mut you = match read_line!("Who are you? ") {
        Ok(name) => Player::new(name),
        Err(_) => {
            main();
            return;
        }
    };

    let mut opponent = match read_line!("Who is your opponent? ") {
        Ok(name) => Player::new(name),
        Err(_) => {
            main();
            return;
        }
    };

    let mut rng = rand::rng();
    let mut round = 1;

    while game(&round, &mut you, &mut opponent, &mut rng) {
        round += 1;
    }

    println!("\n--- {} ---\n", style::win(&"Results"));

    println!("{}(score: {}, defeats: {})", style::player_one(&you), you.score.yellow(), you.defeats.red());
    println!("{}(score: {}, defeats: {})", style::player_two(&opponent), opponent.score.yellow(), opponent.defeats.red());

    #[cfg(not(debug_assertions))]
    pause!();
}

fn game(round: &i32, you: &mut Player, opponent: &mut Player, rng: &mut ThreadRng) -> bool {
    println!("\n--- {}{} ---\n", style::round(&"Round: "), style::round(round));

    println!("0. Rock\n1. Paper\n2. Scissor");

    you.play = match read_number!("Choose: ") {
        Ok(number) => Play::try_from(number).unwrap_or(Play::Rock),
        Err(_) => {
            println!("This is not a number!");
            return true;
        }
    };

    opponent.play = Play::try_from(rng.random_range(0..=2)).unwrap_or(Play::Rock);
    println!(
        "\n{} played {} and {} played {}", 
        style::player_one(you), 
        you.play, 
        style::player_two(opponent), 
        opponent.play
    );

    if you.fight(opponent) {
        you.score += 1;
        opponent.defeats += 1;

        println!("{}{}", style::win(you), WIN_TEXT);
    } else if opponent.fight(you) {
        opponent.score += 1;
        you.defeats += 1;

        println!("{}{}", style::win(opponent), WIN_TEXT);
    } else {
        println!("Draw...");
    }

    match read_number!("Want restart (1. Yes, 0. No)? ") {
        Ok(choose) => choose == 1,
        Err(_) => false
    }
}