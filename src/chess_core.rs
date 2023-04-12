use std::fmt::{Display, Formatter};
use crate::chess_common::*;

pub type BoardSquares = [[Square; 8]; 8];
pub struct Board {
    squares: BoardSquares,
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            squares: [[Square {piece: None}; 8]; 8]
        };
        b.new_game();
        b
    }

    pub fn new_game(&mut self) {
        // Add pawns
        for f in 0..8 {
            self.squares[ChessRank::R2.as_usize()][f] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Pawn)));
            self.squares[ChessRank::R7.as_usize()][f] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Pawn)));
        }

        // Add Rooks
        self.squares[ChessRank::R1.as_usize()][ChessFile::A.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Rook)));
        self.squares[ChessRank::R1.as_usize()][ChessFile::H.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Rook)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::A.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Rook)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::H.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Rook)));

        // Add Knights
        self.squares[ChessRank::R1.as_usize()][ChessFile::B.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Knight)));
        self.squares[ChessRank::R1.as_usize()][ChessFile::G.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Knight)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::B.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Knight)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::G.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Knight)));

        // Add Bishops
        self.squares[ChessRank::R1.as_usize()][ChessFile::C.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Bishop)));
        self.squares[ChessRank::R1.as_usize()][ChessFile::F.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Bishop)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::C.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Bishop)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::F.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Bishop)));

        // Add Queens
        self.squares[ChessRank::R1.as_usize()][ChessFile::D.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::Queen)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::D.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::Queen)));

        // Add Kings
        self.squares[ChessRank::R1.as_usize()][ChessFile::E.as_usize()] = Square::new(Some(Piece::new(Team::Light, ChessPiece::King)));
        self.squares[ChessRank::R8.as_usize()][ChessFile::E.as_usize()] = Square::new(Some(Piece::new(Team::Dark, ChessPiece::King)));

    }

    pub fn get_squares(&self) -> &BoardSquares {
        &self.squares
    }
}

#[derive(Copy, Clone)]
pub struct Square {
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
    pub fn new(p: Option<Piece>) -> Square {
        Square { piece: p }
    }

    pub fn get_piece(&self) -> &Option<Piece> {
        &self.piece
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    team: Team,
    piece_type: ChessPiece,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_unicode_symbol())
    }
}

impl Piece {
    pub fn new(team: Team, piece_type: ChessPiece) -> Piece {
        Piece {team, piece_type }
    }

    pub fn get_unicode_symbol(self) -> char {
        match self.team {
            Team::Dark => match self.piece_type {
                ChessPiece::Pawn => '\u{265F}', // Some wierd error when copy + pasting char from the web...
                ChessPiece::Knight => '♞',
                ChessPiece::Bishop => '♝',
                ChessPiece::Rook => '♜',
                ChessPiece::Queen => '♛',
                ChessPiece::King => '♚',
            }
            Team::Light => match self.piece_type {
                ChessPiece::Pawn => '♙',
                ChessPiece::Knight => '♘',
                ChessPiece::Bishop => '♗',
                ChessPiece::Rook => '♖',
                ChessPiece::Queen => '♕',
                ChessPiece::King => '♔',
            }
        }
    }

    pub fn get_team(&self) -> &Team {
        &self.team
    }
}

#[derive(Copy, Clone)]
pub enum Team {
    Light,
    Dark,
}
