#[derive(Debug)]
struct TreeNode {
    state: Board,
    children: Option<Vec<TreeNode>>,
    player1_wins: u32,
    games_played: u32,
}

impl TreeNode {
    fn new(state: Board) -> TreeNode {

    }

    /// Repeatedly expands the tree until the maximum duration is reached
    pub fn search(time_limit: Duration) {
        unimplemented!()
    }

    /// Returns the best move from the perspective of the current player
    pub fn best_move() -> [u8; 5] {
        unimplemented!()
    }

    /// Recursively expands the tree once
    fn expand_once(&mut self) {

    }
}
