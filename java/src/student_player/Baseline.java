package student_player;

public class Baseline {
    static {
        System.load(System.getProperty("user.dir") + "/data/libBaseline.so");
    }

    public native long chooseMove(long player1, long player2);
}
