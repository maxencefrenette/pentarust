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

#[cfg(not(feature = "baseline"))]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_student_1player_PentaRust_chooseMove(
    _env: JNIEnv,
    _class: JClass,
    player1: u64,
    player2: u64,
) -> u64 {
    choose_move(player1, player2)
}

#[cfg(feature = "baseline")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_student_1player_Baseline_chooseMove(
    _env: JNIEnv,
    _class: JClass,
    player1: u64,
    player2: u64,
) -> u64 {
    choose_move(player1, player2)
}

fn choose_move(player1: u64, player2: u64) -> u64 {
    let board = Board { player1, player2 };
    let player = board.turn();

    // Check for a winning move
    let winning_state_opt = board.children().into_iter().find(|c| c.player_won(player));
    let action = if let Some(winning_state) = winning_state_opt {
        trace!("Found winning move !");
        board.action_to(winning_state)
    } else {
        let mut tree = TreeNode::new(board);

        tree.search(Duration::from_millis(1_800));
        trace!("{:?} {:?}", player1, player2);
        trace!("{:?}", tree.state.outcome());
        trace!("{:?}", tree.state);
        trace!("{:?}", tree.win_stats);

        tree.best_move()
    };

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
    let player_id = match player {
        Player::Player1 => 0u64,
        Player::Player2 => 1u64,
    };

    x | (y << 8) | (a_swap << 16) | (b_swap << 24) | (player_id << 32)
}
