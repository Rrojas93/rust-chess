use std::{
    fmt::{
        Display, 
        Formatter
    },
    io::{Write}, process::Command,
};
use crate::{
    chess_core::{
        Board,
        Team
    },
    chess_command::{CommandParser, RegisteredCommand, ArgType, ParsedCommand},
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
    let mut parser = register_commands();
    parser.set_description(String::from("Classic chess game in the terminal!"));
    let mut game: Board = Board::new();
    let mut user_input;

    loop {
        println!("{game}");
        print!(">> ");
        std::io::stdout().flush();
        user_input = get_user_input();
        if let Some(cmd) = parser.parse_string(user_input) {
            match cmd.get_id() {
                ChessTuiCommands::Move => {
                    println!("Entered a move.");
                },
                ChessTuiCommands::Undo => {
                    println!("Undoing move.");
                },
                ChessTuiCommands::Redo => {
                    println!("Redoing move.");
                },
                ChessTuiCommands::Reset => {
                    println!("Resetting board.");
                    game = Board::new();
                },
                ChessTuiCommands::Save => {
                    println!("Saving PGN to file.");
                },
                ChessTuiCommands::Load => {
                    println!("Loading game from PGN file.");
                },
                ChessTuiCommands::Quit => {
                    println!("Quiting game.");
                    break;
                },
                ChessTuiCommands::Help => {
                    std::io::stdout().write(parser.get_help_text().as_bytes()).unwrap();
                }
            }
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

fn register_commands() -> CommandParser<ChessTuiCommands> {
    CommandParser::from(
        vec![
            RegisteredCommand::new(ChessTuiCommands::Move)
                .add_aliases(&["move", "m"])
                .add_help_str("Perform a move using PGN chess notation.")
                .add_num_args(1)
                .add_arg_type(ArgType::ArgType_String)
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Undo)
                .add_aliases(&["undo", "u"])
                .add_help_str("Undo the last move.")
                .add_num_args(1)
                .add_arg_type(ArgType::ArgType_u32)
                .add_default_args_u32(vec![1])
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Redo)
                .add_aliases(&["redo", "r"])
                .add_help_str("Redo the last move.")
                .add_num_args(1)
                .add_arg_type(ArgType::ArgType_u32)
                .add_default_args_u32(vec![1])
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Reset)
                .add_aliases(&["reset", "rs"])
                .add_help_str("Reset the board.")
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Save)
                .add_aliases(&["save", "s"])
                .add_help_str("Save the game in PGN format.")
                .add_num_args(1)
                .add_arg_type(ArgType::ArgType_String)
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Load)
                .add_aliases(&["load", "l"])
                .add_help_str("Load a game from PGN format file.")
                .add_num_args(1)
                .add_arg_type(ArgType::ArgType_String)
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Quit)
                .add_aliases(&["quit", "q"])
                .add_help_str("Quit the game. Warning: Unsaved game progress will be lost.")
                .build().unwrap(),
            RegisteredCommand::new(ChessTuiCommands::Help)
                .add_aliases(&["help", "h"])
                .add_help_str("Prints help text with available commands.")
                .build().unwrap(),
        ]
    )
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
            output.push_str(format!("{} ", r).as_str());

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