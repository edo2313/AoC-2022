@echo off
set day=%date:~0,2%
cargo new day%day%
aoc d -s .\.session -I -i day%day%\input.txt