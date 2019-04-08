#!/bin/bash

OUTCOMES_FILE=java/logs/outcomes.txt
NUM_GAMES=20

cd java
java -cp bin autoplay.Autoplay $NUM_GAMES
cd ..

player1_wins=$(tail -n $NUM_GAMES $OUTCOMES_FILE | awk '/260685124,baseline,0/{++cnt} END {print cnt}')
player2_wins=$(tail -n $NUM_GAMES $OUTCOMES_FILE | awk '/baseline,260685124,1/{++cnt} END {print cnt}')

echo "Win rate as white: $((200 * $player1_wins / $NUM_GAMES))%";
echo "Win rate as black: $((200 * $player2_wins / $NUM_GAMES))%";
