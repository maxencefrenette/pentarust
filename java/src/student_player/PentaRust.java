package student_player;

public class PentaRust {
    static {
        try {
            System.load(System.getProperty("user.dir") + "/data/libPentaRust.so");
        } catch (Exception e) {
            System.err.println(e);
            throw e;
        }
    }

    public native long chooseMove(long player1, long player2);
}
