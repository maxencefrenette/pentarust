use crate::game::Board;
use crate::game::Player;

pub fn eval(board: Board) -> i32 {
    let mut eval = 0;

    const CENTER_BONUS: i32 = 100;
    const CENTERS: u64 = 0b000000_010010_000000_000000_010010_000000;

    eval += (board.player1 & CENTERS).count_ones() as i32 * CENTER_BONUS;
    eval -= (board.player2 & CENTERS).count_ones() as i32 * CENTER_BONUS;

    if board.turn() == Player::Player1 {
        eval
    } else {
        -eval
    }
}
