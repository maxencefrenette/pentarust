use crate::game::Action;
use crate::game::Board;
use crate::game::Outcome;
use crate::win_stats::WinStats;
use float_ord::FloatOrd;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use rand::Rng;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct TreeNode {
    pub state: Board,
    pub children: Option<Vec<TreeNode>>,
    pub win_stats: WinStats,
}

impl TreeNode {
    pub fn new(state: Board) -> TreeNode {
        TreeNode {
            state,
            children: None,
            win_stats: Default::default(),
        }
    }

    /// Repeatedly expands the tree until the maximum duration is reached
    pub fn search(&mut self, time_limit: Duration) {
        const WARMUP_GAMES: u8 = 100;
        let start = SystemTime::now();
        let mut games_played: f32 = f32::from(WARMUP_GAMES);
        let mut time_elapsed = Duration::default();

        for _ in 0..WARMUP_GAMES {
            self.expand(1_000_000.);
        }

        while time_elapsed < time_limit {
            let time_elapsed_f32 = time_elapsed.as_micros() as f32;
            let time_remaining = (time_limit - time_elapsed).as_micros() as f32;
            self.expand(games_played * time_remaining / time_elapsed_f32);

            games_played += 1.;
            time_elapsed = start.elapsed().unwrap_or_else(|_| Duration::new(0, 0));
        }
    }

    /// Expands this node once
    fn expand(&mut self, remaining_games: f32) -> Outcome {
        let mut rng = thread_rng();

        let winner = if let Some(children) = &mut self.children {
            let player = self.state.turn();
            let next = children
                .iter_mut()
                .max_by_key(|child| {
                    FloatOrd(
                        child
                            .win_stats
                            .upper_confidence_bound(player, remaining_games)
                            + 0.0001f32 * rng.gen::<f32>(),
                    )
                })
                .expect("non-empty list");

            next.expand(remaining_games)
        } else {
            if let Some(outcome) = self.state.outcome() {
                return outcome;
            }

            self.children = Some(
                self.state
                    .children()
                    .into_iter()
                    .map(TreeNode::new)
                    .collect(),
            );

            self.simulate()
        };

        self.win_stats.update_score(winner);
        winner
    }

    /// Simulate a game with random moves
    fn simulate(&self) -> Outcome {
        let mut rng = thread_rng();
        let mut state = self.state;
        let mut player = state.turn();

        loop {
            if let Some(outcome) = state.outcome() {
                return outcome;
            }

            state = *state
                .children()
                .iter()
                .choose(&mut rng)
                .expect("all non-terminal states have children");
            player = player.opponent();
        }
    }

    /// Returns the best move from the perspective of the current player
    pub fn best_move(&self) -> Action {
        let player = self.state.turn();
        let actions = self.state.actions();
        let best_move_index = self
            .children
            .as_ref()
            .expect("non-empty tree")
            .iter()
            .enumerate()
            .max_by_key(|(_, child)| FloatOrd(child.win_stats.expected_win_ratio(player)))
            .expect("non-empty list")
            .0;

        actions[best_move_index]
    }
}
