#!/bin/bash

rm -rf submission/
rm -f submission.zip

mkdir submission
mkdir submission/src
mkdir submission/src/student_player
cp java/src/student_player/StudentPlayer.java submission/src/student_player/StudentPlayer.java
cp java/src/student_player/PentaRust.java submission/src/student_player/PentaRust.java
mkdir submission/data
cp target/release/libpentarust.so submission/data/libPentaRust.so
zip -r submission.zip submission

rm -rf submission/
