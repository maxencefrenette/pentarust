use crate::alpha_beta::search;
use crate::alpha_beta::TranspositionTable;
use crate::game::Action;
use crate::game::Board;
use crate::game::Player;
use crate::game::Swap;
use jni::objects::JClass;
use jni::JNIEnv;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::Duration;

// Todo: only export for the test target
pub mod alpha_beta;
pub mod game;
pub mod mcts;

const TOP_LEFT: u64 = 0;
const TOP_RIGHT: u64 = 1;
const BOTTOM_LEFT: u64 = 2;
const BOTTOM_RIGHT: u64 = 3;

#[cfg(not(feature = "trace"))]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "trace")]
macro_rules! trace {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
    })
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_student_1player_PentaRust_chooseMove(
    _env: JNIEnv,
    _class: JClass,
    player1: u64,
    player2: u64,
) -> u64 {
    choose_move(player1, player2)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_student_1player_Baseline_chooseMove(
    _env: JNIEnv,
    _class: JClass,
    player1: u64,
    player2: u64,
) -> u64 {
    choose_move(player1, player2)
}

pub fn choose_move(player1: u64, player2: u64) -> u64 {
    let board = Board { player1, player2 };
    let action = best_move(board, Duration::from_millis(1_800));

    trace!("{:?}", action);

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

/*
pub fn best_move(board: Board, search_time: Duration) -> Action {
    let player = board.turn();

    // Check for a winning move
    let winning_state_opt = board
        .children(false)
        .into_iter()
        .find(|c| c.player_won(player));

    if let Some(winning_state) = winning_state_opt {
        trace!("Found winning move !");
        board.action_to(winning_state)
    } else {
        let mut tree = TreeNode::new(board);

        tree.search(search_time);
        trace!("{:?} {:?}", player1, player2);
        trace!("{:?}", tree.state.outcome());
        trace!("{:?}", tree.state);
        trace!("{:?}", tree.win_stats);

        tree.best_move()
    }
}
*/

lazy_static! {
    static ref TRANSPO_TABLE: Mutex<TranspositionTable> =
        Mutex::new(TranspositionTable::new(1_000_000));
}

pub fn best_move(board: Board, search_time: Duration) -> Action {
    let transpo_table = &mut TRANSPO_TABLE.lock().expect("failed to lock transpo table");

    search(board, search_time, transpo_table)
}
