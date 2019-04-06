use jni::JNIEnv;

// Todo: only export for the test target
pub mod game;
pub mod mcts;
pub mod win_stats;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_student_1player_PentaRust_chooseMove(
    env: JNIEnv,
    player1: u64,
    player2: u64,
) -> u64 {
    0x0000000000010000
}
