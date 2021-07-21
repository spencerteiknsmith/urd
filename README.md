# URD
This little project interactively takes text and translates it to "urd" text. The idea was spawned in a BYU CS Discord server, where somebody accidentally typed "urd" instead of "yes" (by having their hands transposed one position on the keyboard). This program can also translate back from urd text to normal text.

This project was primarily a way to get more familiar with rust.

## Edges of the keyboard
The matter of how to deal with the edges of the keyboard were a little tricky. Thanks to Chase for suggesting a wraparound solution. If you have a cylindrical keyboard, this should make perfect sense to you.