use crate::game::Board;
use crate::game::Player;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
struct TreeNode {
    state: Board,
    children: Option<Vec<TreeNode>>,
    player1_wins: u32,
    games_played: u32,
}

impl TreeNode {
    fn new(state: Board) -> TreeNode {
        TreeNode {
            state,
            children: Some(Vec::new()),
            player1_wins: 0,
            games_played: 0,
        }
    }

    /// Repeatedly expands the tree until the maximum duration is reached
    pub fn search(&mut self, time_limit: Duration) {
        let start = SystemTime::now();

        while start.elapsed().unwrap_or_else(|_| Duration::new(0, 0)) < time_limit {
            /*let mut visited = Vec::<&mut TreeNode>::with_capacity(36);
            let cur = &self;

            // Selection
            while let Some(mut children) = &cur.children {
                let next = children
                    .iter_mut()
                    .max_by_key(|child| child.upper_confidence_bound())
                    .expect("non-empty list");
                visited.push(*cur);
                cur = &next;
            }

            // if cur.state.player_won(true) {}

            // if cur.state.player_won(false) {}

            // Expansion
            cur.expand();
            let next = cur
                .children
                .unwrap()
                .iter_mut()
                .max_by_key(|child| child.upper_confidence_bound())
                .expect("non-empty list");
            visited.push(*cur);
            cur = &next;

            // Simulation

            // Backpropagation*/
        }
    }

    /// Returns the best move from the perspective of the current player
    pub fn best_move() -> [u8; 5] {
        unimplemented!()
    }

    /// Expands this node once
    fn expand(&mut self) {
        // self.children = self
        //     .state
        //     .children()
        //     .into_iter()
        //     .map(|state| TreeNode::new(state))
        //     .collect();
    }

    fn upper_confidence_bound(&self) {
        let w = if self.state.turn() == Player::Player1 {
            self.player1_wins
        } else {
            self.games_played - self.player1_wins
        };
    }

    fn update_score(&mut self, player1_won: bool) {
        self.games_played += 1;
        if player1_won {
            self.player1_wins += 1;
        }
    }
}
