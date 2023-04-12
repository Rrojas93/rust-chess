
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Rust Chess")]
#[command(author = "Raul Rojas")]
#[command(version = "1.0")]
#[command(about = "The game of chess written in Rust!")]
pub struct ChessTuiCmd {
    #[command(subcommand)]
    pub command: ChessCommands,
}

#[derive(Subcommand, Debug)]
pub enum ChessCommands {
    /// Make a chess move.
    #[command(about = "A PGN valid move string.", long_about = "Examples:\n  e4\n  exd5\n  Nc3\n  e8=Q\n  O-O-O")]
    Move { pgn_move: String },
    /// Undo the last move or moves.
    Undo { undo_count: Option<u8> },
    /// Redo the previously undon move or moves.
    Redo { redo_count: Option<u8> },
    /// Reset the board.
    Reset,
    /// Save the current game into a PGN file.
    Save { file_path: String },
    /// Load a game from a PGN file.
    Load { file_path: String },
    /// Quit the game. Warning: Unsaved progress will be lost.
    Quit,
}