@echo off
set day=%date:~0,2%
set day=10
cargo new day%day%
aoc d -s .\.session -I -i day%day%\input.txt -d %day%