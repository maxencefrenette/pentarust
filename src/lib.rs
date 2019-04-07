use crate::game::Board;
use crate::game::Player;
use crate::game::Swap;
use crate::mcts::TreeNode;
use jni::objects::JClass;
use jni::JNIEnv;
use std::time::Duration;

// Todo: only export for the test target
pub mod game;
pub mod mcts;
pub mod win_stats;

const TOP_LEFT: u64 = 0;
const TOP_RIGHT: u64 = 1;
const BOTTOM_LEFT: u64 = 2;
const BOTTOM_RIGHT: u64 = 3;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_student_1player_PentaRust_chooseMove(
    _env: JNIEnv,
    _class: JClass,
    player1: u64,
    player2: u64,
) -> u64 {
    let board = Board { player1, player2 };
    let mut tree = TreeNode::new(board);

    tree.search(Duration::from_millis(1_900));
    eprintln!("{:?} {:?}", player1, player2);
    eprintln!("{:?}", tree.state.outcome());
    eprintln!("{:?}", tree.state);
    eprintln!("{:?}", tree.win_stats);

    let action = tree.best_move();
    eprintln!("{:?}", action);

    let x = u64::from(action.square % 6);
    let y = u64::from(action.square / 6);
    let (a_swap, b_swap) = match action.swap {
        Swap::TL_TR => (TOP_LEFT, TOP_RIGHT),
        Swap::BL_BR => (BOTTOM_LEFT, BOTTOM_RIGHT),
        Swap::TL_BL => (TOP_LEFT, BOTTOM_LEFT),
        Swap::TR_BR => (TOP_RIGHT, BOTTOM_RIGHT),
        Swap::TL_BR => (TOP_LEFT, BOTTOM_RIGHT),
        Swap::TR_BL => (TOP_RIGHT, BOTTOM_LEFT),
    };
    let player_id = match board.turn() {
        Player::Player1 => 0u64,
        Player::Player2 => 1u64,
    };

    x | (y << 8) | (a_swap << 16) | (b_swap << 24) | (player_id << 32)
}
