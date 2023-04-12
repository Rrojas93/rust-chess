#![allow(dead_code)]
mod chess_core;
mod chess_ui;
mod chess_pgn;
mod chess_cmd;
mod chess_common;

use chess_ui::*;

fn main() {
    ui_main();
}