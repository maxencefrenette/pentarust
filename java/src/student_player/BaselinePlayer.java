package student_player;

import boardgame.Move;
import pentago_swap.PentagoPlayer;
import pentago_swap.PentagoBoardState;
import pentago_swap.PentagoBoardState.Piece;
import pentago_swap.PentagoBoardState.Quadrant;
import pentago_swap.PentagoMove;

/** A player file submitted by a student. */
public class BaselinePlayer extends PentagoPlayer {
    private Baseline baseline;

    /**
     * You must modify this constructor to return your student number. This is
     * important, because this is what the code that runs the competition uses to
     * associate you with your agent. The constructor should do nothing else.
     */
    public BaselinePlayer() {
        super("baseline");
    }

    /**
     * This is the primary method that you need to implement. The ``boardState``
     * object contains the current state of the game, which your agent must use to
     * make decisions.
     */
    public Move chooseMove(PentagoBoardState boardState) {
    	if (baseline == null) {
            baseline = new Baseline();
    	}
    	
    	// System.out.println(boardState);
    	
    	long player1 = 0;
    	long player2 = 0;
    	
    	for(int i = 0; i < 6; i++) {
    		for(int j = 0; j < 6; j++) {
        		if (boardState.getPieceAt(i, j) == Piece.WHITE) {
        			player1 |= 1L << (6*i + j);
        		} else if (boardState.getPieceAt(i, j) == Piece.BLACK) {
        			player2 |= 1L << (6*i + j);
        		}
        	}
    	}
    	
    	// System.out.print(player1);
    	// System.out.print(" ");
    	// System.out.println(player2);
    	
        long bitMove = baseline.chooseMove(player1, player2);
        long mask = 0xFF;
        
        int x = (int) (bitMove & mask);
        int y = (int) (bitMove >> 8 & mask);
        Quadrant aSwap = Quadrant.values()[(int) (bitMove >> 16 & mask)];
        Quadrant bSwap = Quadrant.values()[(int) (bitMove >> 24 & mask)];
        int playerId = (int) (bitMove >> 32 & mask);

        return new PentagoMove(y, x, aSwap, bSwap, playerId);
    }
}
