use crate::game::fight_result::FightResult;
use crate::game::play::Play;

pub struct Player {
    pub name: String,
    pub play: Play,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            play: Play::Rock,
        }
    }

    pub fn new_with_play(name: String, play: Play) -> Self {
        Self { name, play }
    }

    pub fn fight<'a>(&'a self, opponent: &'a Player) -> FightResult<'a> {
        match (self.play, opponent.play) {
            (Play::Rock, Play::Scissor)
            | (Play::Paper, Play::Rock)
            | (Play::Scissor, Play::Paper) => FightResult::Win(self),

            (Play::Scissor, Play::Rock)
            | (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissor) => FightResult::Win(opponent),

            (play, _) => FightResult::Draw(play),
        }
    }
}
