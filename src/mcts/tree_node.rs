use crate::game::Action;
use crate::game::Board;
use crate::game::Outcome;
use crate::mcts::WinStats;
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
        let start = SystemTime::now();

        while start.elapsed().unwrap_or_else(|_| Duration::new(0, 0)) < time_limit {
            self.expand(0);
        }
    }

    /// Expands this node once
    fn expand(&mut self, depth: u8) -> Outcome {
        let mut rng = thread_rng();

        let winner = if let Some(children) = &mut self.children {
            let player = self.state.turn();
            let games_played = self.win_stats.games_played;
            let next = children
                .iter_mut()
                .max_by_key(|child| {
                    FloatOrd(
                        child.win_stats.upper_confidence_bound(player, games_played)
                            + 0.0001f32 * rng.gen::<f32>(),
                    )
                })
                .expect("Called TreeNode::expand() on a node with an empty list of children");

            next.expand(depth + 1)
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
        let best_child = self
            .children
            .as_ref()
            .expect("Called TreeNode::best_move() on an empty tree")
            .iter()
            .max_by_key(|child| FloatOrd(child.win_stats.expected_win_ratio(player)))
            .expect("Called TreeNode::best_move() on a node with no children");

        self.state.action_to(best_child.state)
    }
}
