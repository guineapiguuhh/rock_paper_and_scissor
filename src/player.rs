use std::fmt::Display;

// Complexidade desnecessario
// por isso que amo essa linguagem (só usando por 3 dias)
// PORRA

#[derive(PartialEq)]
pub enum Play {
    Rock = 0,
    Paper = 1,
    Scissor = 2
}

impl Play {
    pub fn fight(&self, play: &Play) -> bool {
        *self == Self::Rock && *play == Self::Scissor
        || *self == Self::Paper && *play == Self::Rock
        || *self == Self::Scissor && *play == Self::Paper
    }
}

impl TryFrom<i32> for Play  {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rock),
            1 => Ok(Self::Paper),
            2 => Ok(Self::Scissor),
            _ => Err(())
        }
    }
}

impl Into<i32> for Play {
    fn into(self) -> i32 {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissor => 2,
        }
    }
}

impl Display for Play  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Play::Rock => "rock",
            Play::Paper => "paper",
            Play::Scissor => "scissor",
        })
    }
}

pub struct Player {
    pub name: String,
    pub play: Play,

    pub score: u32,
    pub defeats: u32
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            play: Play::Rock,
            score: 0,
            defeats: 0
        }
    }

    pub fn fight(&self, opponent: &Player) -> bool {
        self.play.fight(&opponent.play)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}