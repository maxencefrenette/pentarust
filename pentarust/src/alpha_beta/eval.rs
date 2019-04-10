use crate::game::Board;
use crate::game::Player;

pub fn eval(board: Board) -> i32 {
    let mut eval = 0;

    const CENTER_BONUS: i32 = 100;
    const CENTERS: [u64; 4] = [
        0b000000_010000_000000_000000_000000_000000,
        0b000000_000010_000000_000000_000000_000000,
        0b000000_000000_000000_000000_010000_000000,
        0b000000_000000_000000_000000_000000_000010,
    ];

    for c in CENTERS.iter() {
        if board.player1 & c != 0 {
            eval += CENTER_BONUS;
        } else if board.player2 & c != 0 {
            eval -= CENTER_BONUS;
        }
    }

    if board.turn() == Player::Player1 {
        eval
    } else {
        -eval
    }
}
