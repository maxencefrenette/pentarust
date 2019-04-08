use crate::game::Player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Player1Win,
    Player2Win,
    Draw,
}

impl Outcome {
    pub fn win(player: Player) -> Outcome {
        match player {
            Player::Player1 => Outcome::Player1Win,
            Player::Player2 => Outcome::Player2Win,
        }
    }
}
