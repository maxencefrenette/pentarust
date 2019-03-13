use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_student_1player_PentaRust_chooseMove(env: JNIEnv, player1: u64, player2: u64) -> u64 {
    0x0000000000010000
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
