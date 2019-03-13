package student_player;

public class PentaRust {
    static {
        System.load(System.getProperty("user.dir") + "/data/libPentaRust.so");
    }

    private native long chooseMoveFFI(long player1, long player2);
    public long chooseMove(long player1, long player2) { return chooseMoveFFI(player1, player2); }
}
