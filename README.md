# Rust Chess
A game of chess written in Rust as a Windows desktop application.

# Requirements
This project is meant to provide a way to learn how to develop a rust application
from start to finish in a way that is game related and fun. I will outline the
fundamental requirements that this game will be built for to avoid scope creep 
because I have no discipline in controlling myself from adding more and more 
features. As such, this will outline a minimal viable product as phase 1. More phases
can be added afterwards but should be separate after the MVP is complete to avoid 
distractions.

## Phase 1: Minimal Viable Product (MVP)
1. Two Players should be able to play chess in a turn style game from a single computer.
2. No AI play yet. Meaning a second player is required to really play. Solo players
will only be able to play against themselves or perform self analysis. This should be 
no different from having a physical chess board.
3. Game should be able to keep up with game state for the following requirements: 
    * Keep up with who's turn it is.
    * Ability to undo/redo moves.
    * Record the game played in chess notation (PGN)
        * Output to a file
        * Show as a string
4. The game should enforce the rules of chess and prevent the players from performing
incorrect moves and provide feedback.
5. UI requirements are as follows:
    * The game UI will be terminal based but may be improved to a proper UI in phase 2.
    * User input should be recieved in chess notation from the terminal.
    * Terminal should display pieces and certain PGN notation with chess unicode symbols.
    * Terminal should provide color output for board display and colorization.
    * The board should be shown at all times.
    * The PGN string should be shown at all times as a way to see move history.

6. Terminal Commands:
    * Since the game's UI will be terminal based, user needs to be able to enter
    commands to control the game and move pieces.
    * List of commands:
        * move [PGN move]
        * undo [num]
        * redo [num]
        * reset
        * save [path]
        * load [path]
        * quit
        * help

## Phase 2: Let's make it fun.
1. Integrate stockfish as a chess engine which the player can play against.
2. Implement a proper UI with the following requirements:
    * Drag and drop pieces.
    * When pieces are selected, the UI should show where the piece can move.
    * ...