use std::fmt::{Display, Formatter};
use std::io::{self, Write};


fn main() {
    let game: Board = Board::new();
    println!("{game}");
    print!(">> ");

    io::stdout().flush();

    let commands = vec![
        RegisteredCommand::new(String::from("move"), Command::Move(None)),
        RegisteredCommand::new(String::from("undo"), Command::Undo(None)),
        RegisteredCommand::new(String::from("redo"), Command::Redo(None)),
        RegisteredCommand::new(String::from("reset"), Command::Reset),
        RegisteredCommand::new(String::from("save"), Command::Save(None)),
        RegisteredCommand::new(String::from("quit"), Command::Quit),
    ];

    let mut user_input = String::new();
    io::stdin()
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

struct Board {
    squares: [[Square; 8]; 8],
}

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

fn terminal_fg_color_256(c: u8) -> String {
    format!("\u{001b}[38;5;{c}m")
}

fn terminal_bg_color_256(c: u8) -> String {
    format!("\u{001b}[48;5;{c}m")
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for r in (0..self.squares.len()).rev() {
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
            for f in 0..self.squares[r].len() {
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
                if let Some(p) = self.squares[r][f].piece {
                    match p.team {
                        Team::Dark => {
                            output.push_str(dark_fg_color.as_str());
                        }
                        Team::Light => {
                            output.push_str(light_fg_color.as_str());
                        }
                    }
                }

                output.push_str(format!(" {} ", self.squares[r][f]).as_str());
            }
        }
        output.push_str(format!("{}\n  ", TERMINAL_COLOR_RESET).as_str());
        output.push_str(" A  B  C  D  E  F  G  H\n");
        write!(f, "{}", output)
    }
}

impl Board {
    fn new() -> Board {
        let mut b = Board {
            squares: [[Square {piece: None}; 8]; 8]
        };
        b.new_game();
        b
    }

    fn new_game(&mut self) {
        // Add pawns
        for f in 0..8 {
            self.squares[Rank::R2.as_usize()][f] = Square::new(Some(Piece::new(Team::Light, PieceType::Pawn)));
            self.squares[Rank::R7.as_usize()][f] = Square::new(Some(Piece::new(Team::Dark, PieceType::Pawn)));
        }

        // Add Rooks
        self.squares[Rank::R1.as_usize()][File::FA.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Rook)));
        self.squares[Rank::R1.as_usize()][File::FH.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Rook)));
        self.squares[Rank::R8.as_usize()][File::FA.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Rook)));
        self.squares[Rank::R8.as_usize()][File::FH.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Rook)));

        // Add Knights
        self.squares[Rank::R1.as_usize()][File::FB.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Knight)));
        self.squares[Rank::R1.as_usize()][File::FG.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Knight)));
        self.squares[Rank::R8.as_usize()][File::FB.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Knight)));
        self.squares[Rank::R8.as_usize()][File::FG.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Knight)));

        // Add Bishops
        self.squares[Rank::R1.as_usize()][File::FC.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Bishop)));
        self.squares[Rank::R1.as_usize()][File::FF.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Bishop)));
        self.squares[Rank::R8.as_usize()][File::FC.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Bishop)));
        self.squares[Rank::R8.as_usize()][File::FF.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Bishop)));

        // Add Queens
        self.squares[Rank::R1.as_usize()][File::FD.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::Queen)));
        self.squares[Rank::R8.as_usize()][File::FD.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::Queen)));

        // Add Kings
        self.squares[Rank::R1.as_usize()][File::FE.as_usize()] = Square::new(Some(Piece::new(Team::Light, PieceType::King)));
        self.squares[Rank::R8.as_usize()][File::FE.as_usize()] = Square::new(Some(Piece::new(Team::Dark, PieceType::King)));

    }
}

#[derive(Copy, Clone)]
struct Square {
    piece: Option<Piece>
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.piece {
            Some(p) => {
                write!(f, "{}", p)
            }
            None => {
                write!(f, "{}", ' ')
            }
        }
    }
}

impl Square {
    fn new(p: Option<Piece>) -> Square {
        Square { piece: p }
    }
}

#[derive(Copy, Clone)]
struct Piece {
    team: Team,
    piece_type: PieceType,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

impl Piece {
    pub fn new(team: Team, piece_type: PieceType) -> Piece {
        Piece {team, piece_type }
    }

    fn get_symbol(self) -> char {
        match self.team {
            Team::Dark => match self.piece_type {
                PieceType::Pawn => '\u{265F}', // Some wierd error when copy + pasting char from the web...
                PieceType::Knight => '♞',
                PieceType::Bishop => '♝',
                PieceType::Rook => '♜',
                PieceType::Queen => '♛',
                PieceType::King => '♚',
            }
            Team::Light => match self.piece_type {
                PieceType::Pawn => '♙',
                PieceType::Knight => '♘',
                PieceType::Bishop => '♗',
                PieceType::Rook => '♖',
                PieceType::Queen => '♕',
                PieceType::King => '♔',
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Team {
    Light,
    Dark,
}

#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum Rank {
    R1 = 0,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8
}

impl Rank {
    fn as_usize(self) -> usize {
        self as usize
    }
}

enum File {
    FA = 0,
    FB,
    FC,
    FD,
    FE,
    FF,
    FG,
    FH,
}

impl File {
    fn as_usize(self) -> usize {
        self as usize
    }
}

enum Command {
    Move(Option<String>),
    Undo(Option<u32>),
    Redo(Option<u32>),
    Reset,
    Save(Option<String>),
    Quit,
}

fn parse_command(commands: Vec<RegisteredCommand>, user_input: String) -> Option<Command> {
    let mut iter = user_input.trim().split_whitespace();
    if let Some(input_cmd) = iter.next() {
        for registered_cmd in commands {
            if registered_cmd.compare(input_cmd) {
                let cmd_type = registered_cmd.get_cmd_type();
                let arg = iter.next();
                match cmd_type {
                    Command::Move(_) => {
                        if let Some(a) = arg {
                            return Some(Command::Move(Some(String::from(a))));
                        }
                    },
                    Command::Undo(_) => {
                        if let Some(a) = arg {
                            if let Ok(num) = a.parse::<u32>() {
                                return Some(Command::Undo(Some(num)));
                            }
                        }
                    },
                    Command::Redo(_) => {
                        if let Some(a) = arg {
                            if let Ok(num) = a.parse::<u32>() {
                                return Some(Command::Redo(Some(num)));
                            }
                        }
                    },
                    Command::Save(_) => {
                        if let Some(a) = arg {
                            return Some(Command::Save(Some(String::from(a))));
                        }
                    },
                    Command::Reset => return Some(Command::Reset),
                    Command::Quit => return Some(Command::Quit),
                }
            }
        }
    }
    None
}

struct RegisteredCommand {
    cmd_str: String,
    cmd_type: Command,
}

impl RegisteredCommand {
    fn new(cmd_str: String, cmd_type: Command) -> RegisteredCommand {
        RegisteredCommand { cmd_str, cmd_type }
    }

    fn compare(&self, other_str: &str) -> bool {
        self.cmd_str == other_str
    }

    fn get_cmd_type(&self) -> &Command {
        &self.cmd_type
    }
}