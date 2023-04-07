#![allow(dead_code)]
mod chess_core;
mod chess_command;
mod chess_ui;
mod chess_pgn;

use chess_ui::*;

fn main() {
    ui_main();
}