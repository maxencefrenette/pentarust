package student_player;

import boardgame.Move;
import pentago_swap.PentagoPlayer;
import pentago_swap.PentagoBoardState;
import pentago_swap.PentagoBoardState.Piece;
import pentago_swap.PentagoBoardState.Quadrant;
import pentago_swap.PentagoMove;

/** A player file submitted by a student. */
public class StudentPlayer extends PentagoPlayer {
    private PentaRust pentaRust;

    /**
     * You must modify this constructor to return your student number. This is
     * important, because this is what the code that runs the competition uses to
     * associate you with your agent. The constructor should do nothing else.
     */
    public StudentPlayer() {
        super("260685124");
        pentaRust = new PentaRust();
    }

    /**
     * This is the primary method that you need to implement. The ``boardState``
     * object contains the current state of the game, which your agent must use to
     * make decisions.
     */
    public Move chooseMove(PentagoBoardState boardState) {
    	long player1 = 0;
    	long player2 = 0;
    	
    	for(int i = 0; i < 6; i++) {
    		for(int j = 0; j < 6; j++) {
        		if (boardState.getPieceAt(j, i) == Piece.WHITE) {
        			player1 &= 1 << (6*j + i);
        		} else if (boardState.getPieceAt(j, i) == Piece.BLACK) {
        			player2 &= 1 << (6*j + i);
        		}
        	}
    	}
    	
        long bitMove = pentaRust.chooseMove(player1, player2);
        long mask = 0xFF;
        
        int x = (int) (bitMove & mask);
        int y = (int) (bitMove >> 8 & mask);
        Quadrant aSwap = Quadrant.values()[(int) (bitMove >> 16 & mask)];
        Quadrant bSwap = Quadrant.values()[(int) (bitMove >> 24 & mask)];
        int playerId = (int) (bitMove >> 32 & mask);

        // Return random move for now
        PentagoMove m = new PentagoMove(x, y, aSwap, bSwap, playerId);
        System.out.println(m.toPrettyString());
        return m;
    }
}
