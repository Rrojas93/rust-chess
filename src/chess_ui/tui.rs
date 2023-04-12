use std::{
    fmt::{
        Display,
        Formatter
    },
    io::{Write},
};
use clap::Parser;

use crate::{
    chess_core::{
        Board,
        Team
    },
    chess_cmd::{ChessTuiCmd, ChessCommands},
    chess_pgn::{PgnMove, ChessMove},
};

const TERMINAL_COLOR_RESET: &str        = "\u{001b}[0m";
const TERMINAL_FG_COLOR_BLACK: &str     = "\u{001b}[30m";
const TERMINAL_FG_COLOR_RED: &str       = "\u{001b}[31m";
const TERMINAL_FG_COLOR_GREEN: &str     = "\u{001b}[32m";
const TERMINAL_FG_COLOR_YELLOW: &str    = "\u{001b}[33m";
const TERMINAL_FG_COLOR_BLUE: &str      = "\u{001b}[34m";
const TERMINAL_FG_COLOR_MAGENTA: &str   = "\u{001b}[35m";
const TERMINAL_FG_COLOR_CYAN: &str      = "\u{001b}[36m";
const TERMINAL_FG_COLOR_WHITE: &str     = "\u{001b}[37m";

const TERMINAL_BG_COLOR_BLACK: &str     = "\u{001b}[40m";
const TERMINAL_BG_COLOR_RED: &str       = "\u{001b}[41m";
const TERMINAL_BG_COLOR_GREEN: &str     = "\u{001b}[42m";
const TERMINAL_BG_COLOR_YELLOW: &str    = "\u{001b}[43m";
const TERMINAL_BG_COLOR_BLUE: &str      = "\u{001b}[44m";
const TERMINAL_BG_COLOR_MAGENTA: &str   = "\u{001b}[45m";
const TERMINAL_BG_COLOR_CYAN: &str      = "\u{001b}[46m";
const TERMINAL_BG_COLOR_WHITE: &str     = "\u{001b}[47m";

pub fn tui_main() {
    let mut game: Board = Board::new();
    let mut user_input;

    loop {
        println!("{game}");
        print!(">> ");
        std::io::stdout().flush().unwrap();
        user_input = get_user_input();
        user_input.insert_str(0, ">> ");
        let parse_result = ChessTuiCmd::try_parse_from(user_input.split_whitespace());
        match parse_result {
            Ok(input_cmd) => {
                match input_cmd.command {
                    ChessCommands::Move { pgn_move } => {
                        let parsed_move_result = ChessMove::from(&pgn_move);
                        match parsed_move_result {
                            Ok(parsed_move) => {
                                println!("Entered move: {}", parsed_move);
                            }
                            Err(e) => {
                                println!("Invalid move: {pgn_move}");
                            }
                        }
                    }
                    ChessCommands::Undo { undo_count } => {
                        let num = match undo_count {
                            Some(n) => n,
                            None => 1,
                        };
                        println!("Undoing {} move(s)", num);
                    },
                    ChessCommands::Redo { redo_count } => {
                        let num = match redo_count {
                            Some(n) => n,
                            None => 1,
                        };
                        println!("Redoing {} move(s)", num);
                    },
                    ChessCommands::Reset => {
                        println!("Resetting board.");
                        game.new_game();
                    },
                    ChessCommands::Save { file_path } => {
                        println!("Saving game to file: {}", file_path);
                    },
                    ChessCommands::Load { file_path } => {
                        println!("Loading game from file: {}", file_path);
                    },
                    ChessCommands::Quit => {
                        println!("Quitting game.");
                        break;
                    },
                }
            },
            Err(e) => println!("{e}"),
        }
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input
}

#[derive(Clone, Copy)]
enum ChessTuiCommands {
    Move,
    Undo,
    Redo,
    Reset,
    Save,
    Load,
    Quit,
    Help,
}

fn terminal_fg_color_256(c: u8) -> String {
    format!("\u{001b}[38;5;{c}m")
}

fn terminal_bg_color_256(c: u8) -> String {
    format!("\u{001b}[48;5;{c}m")
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for r in (0..self.get_squares().len()).rev() {
            // reset terminal colorization before newline character to avoid coloring the rest of the line.
            output.push_str(TERMINAL_COLOR_RESET);
            output.push('\n');

            // display the row number
            output.push_str(format!("{} ", r + 1).as_str());

            // Set colorization for the next characters.
            let light_bg_color = terminal_bg_color_256(180);
            let light_fg_color = terminal_fg_color_256(255);
            let dark_bg_color = terminal_bg_color_256(64);
            let dark_fg_color = terminal_fg_color_256(240);
            for f in 0..self.get_squares()[r].len() {
                if r % 2 == 0 {
                    if f % 2 == 0 {
                        output.push_str(dark_bg_color.as_str());
                    }
                    else {
                        output.push_str(light_bg_color.as_str());
                    }
                }
                else {
                    if f % 2 == 0 {
                        output.push_str(light_bg_color.as_str());
                    }
                    else {
                        output.push_str(dark_bg_color.as_str());
                    }
                }
                if let Some(p) = self.get_squares()[r][f].get_piece() {
                    match p.get_team() {
                        Team::Dark => {
                            output.push_str(dark_fg_color.as_str());
                        }
                        Team::Light => {
                            output.push_str(light_fg_color.as_str());
                        }
                    }
                }

                output.push_str(format!(" {} ", self.get_squares()[r][f]).as_str());
            }
        }
        output.push_str(format!("{}\n  ", TERMINAL_COLOR_RESET).as_str());
        output.push_str(" A  B  C  D  E  F  G  H\n");
        write!(f, "{}", output)
    }
}