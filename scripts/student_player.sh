#!/bin/bash

cd java
java -cp bin boardgame.Client student_player.StudentPlayer $1 $2
cd ..
