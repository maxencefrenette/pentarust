use crate::game::Outcome;
use crate::game::Player;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct WinStats {
    pub player1_wins: f32,
    pub player2_wins: f32,
    pub games_played: f32,
}

impl WinStats {
    pub fn expected_win_ratio(self, player: Player) -> f32 {
        if self.games_played == 0. {
            return 0.5;
        }

        let w = self.wins(player);
        let n = self.games_played;

        w / n
    }

    pub fn upper_confidence_bound(self, player: Player, remaining_games: f32) -> f32 {
        if self.games_played == 0. {
            return 1_000_000.;
        }

        let c = f32::sqrt(2.);
        let w = self.wins(player);
        let n = self.games_played;

        (w / n) + c * f32::sqrt(remaining_games.ln() / n)
    }

    fn wins(self, player: Player) -> f32 {
        match player {
            Player::Player1 => self.player1_wins,
            Player::Player2 => self.player2_wins,
        }
    }

    pub fn update_score(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Player1Win => self.player1_wins += 1.,
            Outcome::Player2Win => self.player2_wins += 1.,
            Outcome::Draw => {
                self.player1_wins += 0.5;
                self.player2_wins += 0.5;
            }
        }

        self.games_played += 1.;
    }
}
