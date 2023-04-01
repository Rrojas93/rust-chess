use std::fmt::{Display, Formatter};

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
    piece_type: PieceType,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_unicode_symbol())
    }
}

impl Piece {
    pub fn new(team: Team, piece_type: PieceType) -> Piece {
        Piece {team, piece_type }
    }

    pub fn get_unicode_symbol(self) -> char {
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

    pub fn get_team(&self) -> &Team {
        &self.team
    }
}

#[derive(Copy, Clone)]
pub enum Team {
    Light,
    Dark,
}

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Rank {
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
    pub fn as_usize(self) -> usize {
        self as usize
    }
}

pub enum File {
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
    pub fn as_usize(self) -> usize {
        self as usize
    }
}