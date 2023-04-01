use std::{
    fmt::{
        Display, 
        Formatter
    },
    io::{Write},
};
use crate::{
    chess_core::{
        Board,
        Team
    },
    chess_command::{
        RegisteredCommand,
        Command,
        parse_command,
    },
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
    let game: Board = Board::new();
    println!("{game}");
    print!(">> ");

    std::io::stdout().flush();

    let commands = vec![
        RegisteredCommand::new(String::from("move"), Command::Move(None)),
        RegisteredCommand::new(String::from("undo"), Command::Undo(None)),
        RegisteredCommand::new(String::from("redo"), Command::Redo(None)),
        RegisteredCommand::new(String::from("reset"), Command::Reset),
        RegisteredCommand::new(String::from("save"), Command::Save(None)),
        RegisteredCommand::new(String::from("quit"), Command::Quit),
    ];

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Error: Failed to read line.");

    if let Some(m) = parse_command(commands, user_input) {
        match m {
            Command::Move(m) => {
                if let Some(m) = m {
                    println!("Entered move: {m}")
                }
                else {
                    println!("Missing move argument.")
                }
            },
            _ => println!("Did something!"),
        }
    }
    else {
        println!("No recognized command recieved!");
    }
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