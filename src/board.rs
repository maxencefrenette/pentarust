#[derive(Debug, PartialEq, Eq, Default)]
struct Board {
    player1: u64,
    player2: u64,
}

impl Board {
    /// Returns the board states that can arise from this board after one move
    pub fn children(self) {
        unimplemented!()
    }

    /// Returns the action that lead from one board to the other
    pub fn action_to(&self, other: &Board) -> [u8; 5] {
        unimplemented!()
    }

    /// Returns true of it's player 1's turn, false otherwise
    pub fn turn(&self) -> bool {
        (self.player1 & self.player2).count_ones() % 2 == 0
    }

    /// A simple sanity check
    pub fn assert_valid() {
        assert!(true)
    }
}
