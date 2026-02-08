use owo_colors::OwoColorize;
use rand::{Rng, rng};

use crate::game::{fight_result::FightResult, play::Play, player::Player};

pub mod io_macros;
pub mod game {
    pub mod fight_result;
    pub mod play;
    pub mod player;
}

fn main() {
    let mut rng = rng();

    let mut you = Player::new(match read_line!("What's your name? ") {
        Ok(name) => name,
        Err(_) => panic!("Failed to read line!"),
    });

    let mut opponent = Player::new(match read_line!("What's your opponent's name? ") {
        Ok(name) => name,
        Err(_) => panic!("Failed to read line!"),
    });

    println!(
        "{} V.S. {}",
        you.name.on_red().black(),
        opponent.name.on_blue().black()
    );
    println!();

    println!("0. rock\n1. paper\n2. scissor");
    you.play = match Play::try_from(match read_number!("Make your choice: ") {
        Ok(x) => x,
        Err(_) => panic!("Failed to read the number!"),
    }) {
        Ok(play) => play,
        Err(_) => panic!("Invalid choice!"),
    };
    opponent.play = Play::try_from(rng.random_range(0..=2)).unwrap_or(Play::Rock);
    println!();

    println!("{} played {}", you.name.on_red().black(), you.play);
    println!(
        "{} played {}",
        opponent.name.on_blue().black(),
        opponent.play
    );
    println!();

    match you.fight(&opponent) {
        FightResult::Win(winner) => {
            println!("{} is the winner!", winner.name.on_yellow().black())
        }
        FightResult::Draw(play) => println!("Both played {}, draw...", play),
    }
    println!();
}
