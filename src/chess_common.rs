
use std::{
    fmt::{
        Display
    }
};

pub enum ChessTurn {
    WhiteToMove,
    BlackToMove,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            ChessPiece::Pawn => 'P', // Should never be used but adding for completeness.
            ChessPiece::Knight => 'N',
            ChessPiece::Bishop => 'B',
            ChessPiece::Rook => 'R',
            ChessPiece::Queen => 'Q',
            ChessPiece::King => 'K',
        };

        write!(f, "{}", p)
    }
}

impl ChessPiece {
    pub fn from(c: char) -> Option<Self> {
        match c {
            'N' => Some(ChessPiece::Knight),
            'B' => Some(ChessPiece::Bishop),
            'R' => Some(ChessPiece::Rook),
            'Q' => Some(ChessPiece::Queen),
            'K' => Some(ChessPiece::King),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChessCastle {
    KingsideCastle,
    QueensideCastle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChessCoordinate {
    file: Option<ChessFile>,
    rank: Option<ChessRank>,
}

impl Display for ChessCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if let Some(f) = self.file {
            output += f.to_string().as_str();
        }
        if let Some(r) = self.rank {
            output += r.to_string().as_str();
        }
        write!(f, "{}", output)
    }
}

impl ChessCoordinate {
    pub fn new(chess_file: ChessFile, chess_rank: ChessRank) -> ChessCoordinate {
        ChessCoordinate {
            rank: Some(chess_rank),
            file: Some(chess_file),
        }
    }

    pub fn new_opt(chess_file: Option<ChessFile>, chess_rank: Option<ChessRank>) -> Self {
        ChessCoordinate {
            file: chess_file,
            rank: chess_rank,
        }
    }

    pub fn from_rank(chess_rank: ChessRank) -> ChessCoordinate {
        ChessCoordinate {
            rank: Some(chess_rank),
            file: None,
        }
    }

    pub fn from_file(chess_file: ChessFile) -> ChessCoordinate {
        ChessCoordinate {
            rank: None,
            file: Some(chess_file),
        }
    }

    pub fn empty() -> ChessCoordinate {
        ChessCoordinate { rank: None, file: None }
    }

    pub fn is_empty(&self) -> bool {
        self.rank.is_none() && self.file.is_none()
    }

    pub fn is_partial(&self) -> bool {
        self.rank.is_some() || self.file.is_some()
    }

    pub fn is_complete(&self) -> bool {
        self.rank.is_some() && self.file.is_some()
    }

    pub fn get_rank(&self) -> &Option<ChessRank> {
        &self.rank
    }

    pub fn get_file(&self) -> &Option<ChessFile> {
        &self.file
    }

    pub fn set_rank(&mut self, chess_rank: ChessRank) {
        self.rank = Some(chess_rank);
    }

    pub fn set_file(&mut self, chess_file: ChessFile) {
        self.file = Some(chess_file);
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChessFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Display for ChessFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fl: char = match self {
            ChessFile::A => 'a',
            ChessFile::B => 'b',
            ChessFile::C => 'c',
            ChessFile::D => 'd',
            ChessFile::E => 'e',
            ChessFile::F => 'f',
            ChessFile::G => 'g',
            ChessFile::H => 'h',
        };
        write!(f, "{fl}")
    }
}

impl ChessFile {
    pub fn from(c: char) -> Option<Self>{
        match c {
            'a' => Some(ChessFile::A),
            'b' => Some(ChessFile::B),
            'c' => Some(ChessFile::C),
            'd' => Some(ChessFile::D),
            'e' => Some(ChessFile::E),
            'f' => Some(ChessFile::F),
            'g' => Some(ChessFile::G),
            'h' => Some(ChessFile::H),
            _ => None
        }
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChessRank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Display for ChessRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r: char = match self {
            ChessRank::R1 => '1',
            ChessRank::R2 => '2',
            ChessRank::R3 => '3',
            ChessRank::R4 => '4',
            ChessRank::R5 => '5',
            ChessRank::R6 => '6',
            ChessRank::R7 => '7',
            ChessRank::R8 => '8',
        };
        write!(f, "{r}")
    }
}

impl ChessRank {
    pub fn from(c: char) -> Option<ChessRank> {
        match c {
            '1' => Some(ChessRank::R1),
            '2' => Some(ChessRank::R2),
            '3' => Some(ChessRank::R3),
            '4' => Some(ChessRank::R4),
            '5' => Some(ChessRank::R5),
            '6' => Some(ChessRank::R6),
            '7' => Some(ChessRank::R7),
            '8' => Some(ChessRank::R8),
            _ => None,
        }
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }
}
