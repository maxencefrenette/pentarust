use crate::game::board_symmetries::*;
use crate::game::Action;
use crate::game::Outcome;
use crate::game::Player;
use crate::game::Swap;
use std::fmt;

const MASK: u64 = 0xF_FFFF_FFFF;

#[derive(PartialEq, Eq, Default, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Board {
    pub player1: u64,
    pub player2: u64,
}

impl Board {
    /// Creates a new board from an array of integers. Useful for testing purposes.
    pub fn new(array: [[u8; 6]; 6]) -> Board {
        let mut player1 = 0u64;
        let mut player2 = 0u64;

        #[allow(clippy::needless_range_loop)]
        for i in 0..6 {
            for j in 0..6 {
                match array[i][j] {
                    1 => player1 |= 1 << (6 * i + j),
                    2 => player2 |= 1 << (6 * i + j),
                    _ => {}
                };
            }
        }

        Board { player1, player2 }
    }

    /// Returns the children that can arise from this board state
    pub fn children(self, filter_forced_moves: bool) -> Vec<Board> {
        let player = self.turn();
        let mut free_squares = self.free_squares();
        let mut children = Vec::<Board>::with_capacity((free_squares.count_ones() * 6) as usize);

        let mut square = 0;
        while free_squares != 0 {
            if (free_squares & 1) != 0 {
                for swap in Swap::iterator() {
                    let child = self.play_at(player, square).swap(*swap);
                    children.push(child);

                    // Check if the opponent could win by playing this move
                    if filter_forced_moves {
                        let opponent = player.opponent();
                        let is_opponent_winning_move = self
                            .play_at(opponent, square)
                            .swap(*swap)
                            .player_won(opponent);

                        if is_opponent_winning_move {
                            let stone_placed = self.play_at(player, square);
                            return Swap::iterator()
                                .map(|swap| stone_placed.swap(*swap))
                                .collect();
                        }
                    }
                }
            }

            free_squares >>= 1;
            square += 1;
        }

        children
    }

    /// Returns the canonical equivalent fo this board.
    ///
    /// The canonical board is defined such that all equivalent board have the same canonical board.
    pub fn canonical(self) -> Board {
        let vertical = flip_vertical(self);
        let symmetries = [
            self,
            vertical,
            flip_horizontal(self),
            flip_diagonal(self),
            flip_antidiagonal(self),
            flip_antidiagonal(vertical), // Rotate 90 clockwise
            flip_horizontal(vertical),   // Rotate 180
            flip_diagonal(vertical),     // Rotate 90 anticlockwise
        ];

        *symmetries.iter().min().unwrap()
    }

    /// Returns the action that lead to the given state
    pub fn action_to(self, next_state: Board) -> Action {
        let player = self.turn();
        let mut free_squares = self.free_squares();

        let mut square = 0;
        while free_squares != 0 {
            if (free_squares & 1) != 0 {
                for swap in Swap::iterator() {
                    let child = self.play_at(player, square).swap(*swap);

                    if child == next_state {
                        return Action::new(square, *swap);
                    }
                }
            }

            free_squares >>= 1;
            square += 1;
        }

        panic!("Couldn't find action from {:?} to {:?}", self, next_state);
    }

    /// Applies an action to the current state
    pub fn apply_action(self, action: Action) -> Board {
        self.play_at(self.turn(), action.square).swap(action.swap)
    }

    /// Returns true of it's player 1's turn, false otherwise
    pub fn turn(&self) -> Player {
        if (self.player1 | self.player2).count_ones() % 2 == 0 {
            Player::Player1
        } else {
            Player::Player2
        }
    }

    pub fn turn_number(self) -> u32 {
        (self.player1 | self.player2).count_ones()
    }

    pub fn outcome(self) -> Option<Outcome> {
        if self.is_draw() {
            Some(Outcome::Draw)
        } else if self.player_won(Player::Player1) {
            Some(Outcome::Player1Win)
        } else if self.player_won(Player::Player2) {
            Some(Outcome::Player2Win)
        } else {
            None
        }
    }

    pub fn player_won(&self, player: Player) -> bool {
        const WIDTH: u8 = 6;
        let board = match player {
            Player::Player1 => self.player1,
            Player::Player2 => self.player2,
        };

        // Check rows
        let no_wrapping_board = board & 0b001111_001111_001111_001111_001111_001111;
        let m = no_wrapping_board & (board >> 2);
        if m & (m >> 1) & (m >> 2) != 0 {
            return true;
        }

        // Check columns
        let m = board & board >> (2 * WIDTH);
        if m & (m >> WIDTH) & (m >> (2 * WIDTH)) != 0 {
            return true;
        }

        // Check diagonal 1
        let no_wrapping_board = board & 0b000011_000111_001110_001110_111000_110000;
        let m = no_wrapping_board & no_wrapping_board >> (2 * (WIDTH - 1));
        if m & (m >> (WIDTH - 1)) & (m >> (2 * (WIDTH - 1))) != 0 {
            return true;
        }

        // Check diagonal 2
        let no_wrapping_board = board & 0b110000_111000_011100_001110_000111_000011;
        let m = no_wrapping_board & no_wrapping_board >> (2 * (WIDTH + 1));
        if m & (m >> (WIDTH + 1)) & (m >> (2 * (WIDTH + 1))) != 0 {
            return true;
        }

        false
    }

    fn is_draw(self) -> bool {
        self.player1 | self.player2 == MASK
    }

    fn free_squares(self) -> u64 {
        !(self.player1 | self.player2) & MASK
    }

    fn play_at(mut self, player: Player, square: u8) -> Board {
        let new_marble = 1 << square;

        if player == Player::Player1 {
            self.player1 |= new_marble;
        } else {
            self.player2 |= new_marble;
        }

        self
    }

    fn swap(mut self, swap: Swap) -> Board {
        const QUADRANT_MASK: u64 = 0b111_000_111_000_111;
        const OFFSET_TOP_LEFT: u8 = 0;
        const OFFSET_TOP_RIGHT: u8 = 3;
        const OFFSET_BOTTOM_LEFT: u8 = 18;
        const OFFSET_BOTTOM_RIGHT: u8 = 21;

        let (offset1, offset2) = match swap {
            Swap::TL_TR => (OFFSET_TOP_LEFT, OFFSET_TOP_RIGHT),
            Swap::BL_BR => (OFFSET_BOTTOM_LEFT, OFFSET_BOTTOM_RIGHT),
            Swap::TL_BL => (OFFSET_TOP_LEFT, OFFSET_BOTTOM_LEFT),
            Swap::TR_BR => (OFFSET_TOP_RIGHT, OFFSET_BOTTOM_RIGHT),
            Swap::TL_BR => (OFFSET_TOP_LEFT, OFFSET_BOTTOM_RIGHT),
            Swap::TR_BL => (OFFSET_TOP_RIGHT, OFFSET_BOTTOM_LEFT),
        };

        let mask1 = QUADRANT_MASK << offset1;
        let mask2 = QUADRANT_MASK << offset2;
        let difference = offset2 - offset1;

        // player 1
        let tmp1 = self.player1 & mask1;
        self.player1 &= !mask1;
        self.player1 |= (self.player1 & mask2) >> difference;
        self.player1 &= !mask2;
        self.player1 |= tmp1 << difference;

        // player 2
        let tmp2 = self.player2 & mask1;
        self.player2 &= !mask1;
        self.player2 |= (self.player2 & mask2) >> difference;
        self.player2 &= !mask2;
        self.player2 |= tmp2 << difference;

        self
    }

    /// Check if the board state is valid
    ///
    /// * No bits are set past the 36th
    /// * No bits are set for both player1 and player2
    pub fn is_valid(self) -> bool {
        (self.player1 & !MASK) == 0
            && (self.player2 & !MASK) == 0
            && (self.player1 & self.player2) == 0
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Board::new([")?;

        for i in 0..6 {
            write!(f, "    [")?;

            for j in 0..6 {
                let n: u64 = 6 * i + j;

                if self.player1 & (1u64 << n) != 0 {
                    write!(f, "1")?;
                } else if self.player2 & (1u64 << n) != 0 {
                    write!(f, "2")?;
                } else {
                    write!(f, "0")?;
                }

                if j != 5 {
                    write!(f, ", ")?;
                }
            }

            write!(f, "]")?;
            if i != 5 {
                write!(f, ",")?;
            }
            writeln!(f)?;
        }

        write!(f, "])")?;

        if !self.is_valid() {
            writeln!(f)?;
            write!(
                f,
                "Board {{ player1: {}, player2: {} }}",
                self.player1, self.player2,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ]),
            Board {
                player1: 0,
                player2: 0,
            }
        );

        assert_eq!(
            Board::new([
                [0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 2, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ]),
            Board {
                player1: 0x0_0000_0002,
                player2: 0x0_0800_0000,
            }
        );
    }

    #[test]
    fn canonical_test() {
        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0]
            ])
            .canonical(),
            Board::new([
                [0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn turn_test() {
        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .turn(),
            Player::Player1
        );

        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .turn(),
            Player::Player2
        );
    }

    #[test]
    fn play_at_test() {
        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .play_at(Player::Player1, 7),
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );

        assert_eq!(
            Board::new([
                [1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .play_at(Player::Player2, 7),
            Board::new([
                [1, 0, 0, 0, 0, 0],
                [0, 2, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn swap_test() {
        assert_eq!(
            Board::new([
                [1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .swap(Swap::TL_BL),
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );

        assert_eq!(
            Board::new([
                [1, 1, 1, 0, 0, 0],
                [1, 1, 1, 0, 0, 0],
                [1, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .swap(Swap::TL_BR),
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 1],
                [0, 0, 0, 1, 1, 1],
                [0, 0, 0, 1, 1, 1]
            ])
        );

        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [2, 2, 2, 0, 0, 0],
                [2, 2, 2, 0, 0, 0],
                [2, 2, 2, 0, 0, 0]
            ])
            .swap(Swap::TR_BL),
            Board::new([
                [0, 0, 0, 2, 2, 2],
                [0, 0, 0, 2, 2, 2],
                [0, 0, 0, 2, 2, 2],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn player_won_test() {
        assert_eq!(
            Board::new([
                [1, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .player_won(Player::Player1),
            true
        );

        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 2, 0, 0, 0],
                [0, 0, 2, 0, 0, 0],
                [0, 0, 2, 0, 0, 0],
                [0, 0, 2, 0, 0, 0],
                [0, 0, 2, 0, 0, 0]
            ])
            .player_won(Player::Player2),
            true
        );

        assert_eq!(
            Board::new([
                [0, 1, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
                [0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 1, 0],
                [0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0]
            ])
            .player_won(Player::Player1),
            true
        );

        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 2],
                [0, 0, 0, 0, 2, 0],
                [0, 0, 0, 2, 0, 0],
                [0, 0, 2, 0, 0, 0],
                [0, 2, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .player_won(Player::Player2),
            true
        );
    }

    #[test]
    fn player_won_wrapping() {
        assert_eq!(
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 1],
                [1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .player_won(Player::Player1),
            false
        );

        assert_eq!(
            Board::new([
                [0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 1, 0],
                [0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0]
            ])
            .player_won(Player::Player1),
            false
        );

        assert_eq!(
            Board::new([
                [0, 1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
            .player_won(Player::Player1),
            false
        );
    }
}
