use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum Play {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<i32> for Play {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rock),
            1 => Ok(Self::Paper),
            2 => Ok(Self::Scissor),
            _ => Err(()),
        }
    }
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Play::Rock => "rock",
                Play::Paper => "paper",
                Play::Scissor => "scissor",
            }
        )
    }
}
