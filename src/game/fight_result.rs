use crate::game::{play::Play, player::Player};

pub enum FightResult<'a> {
    Win(&'a Player),
    Draw(Play),
}
