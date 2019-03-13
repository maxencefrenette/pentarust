package student_player;

public class PentaRust {
    static {
        System.load(System.getProperty("user.dir") + "/data/libPentaRust.so");
    }

    public native long chooseMove(long player1, long player2);
}
