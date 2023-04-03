/*
chess_pgn.rs
Module that provides PGN parsing, loading, and saving and Rust idiomatic
structures for PGN game notation.

Follows the PGN Standard Specification and Implementation Guide here:
https://ia802908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt

Example PGN Game:
[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3
O-O 9. h3 Nb8 10. d4 Nbd7 11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15.
Nb1 h6 16. Bh4 c5 17. dxe5 Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21.
Nc4 Nxc4 22. Bxc4 Nb6 23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7
27. Qe3 Qg5 28. Qxg5 hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33.
f3 Bc8 34. Kf2 Bf5 35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5
40. Rd6 Kc5 41. Ra6 Nf2 42. g4 Bd3 43. Re6 1/2-1/2
*/

use std::{fmt::Display, num::ParseIntError};
use time::OffsetDateTime;

struct PgnGame {
    // Required tag pairs
    event: PgnTagPair<String>,
    site: PgnTagPair<String>,
    date: PgnTagPair<PgnDate>,
    round: PgnTagPair<PgnRound>,
    white: PgnTagPair<String>,
    black: PgnTagPair<String>,
    result: PgnTagPair<PgnResult>,

    // Move text
    moves: MoveList
}

impl Display for PgnGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Show required tags
        output += format!("{}\n", self.event).as_str();
        output += format!("{}\n", self.site).as_str();
        output += format!("{}\n", self.date).as_str();
        output += format!("{}\n", self.round).as_str();
        output += format!("{}\n", self.white).as_str();
        output += format!("{}\n", self.black).as_str();
        output += format!("{}\n", self.result).as_str();

        output += "\n";

        // Show move list
        output += self.moves.to_string().as_str();

        // Add result at the end of move list
        let lines = output.split("\n").last();
        if let Some(last_line) = lines {
            if last_line.len() + self.result.to_string().len() >= 80 {
                output += "\n";
            }
            output += self.result.to_string().as_str();
        }

        write!(f, "{}", output)
    }
}

impl PgnGame {
    
}

struct PgnTagPair<T: Display> {
    tag_name: String,
    tag_value: T
}

impl<T: Display> Display for PgnTagPair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} \"{}\"]", self.tag_name, self.tag_value)
    }
}

struct PgnDate {
    year: Option<i32>,
    month: Option<u8>,
    day: Option<u8>,
}

impl Display for PgnDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if let Some(y) = &self.year {
            output += format!("{:01$}", y, 4).as_str();
        }
        else {
            output += "????"
        }
        output += ".";
        if let Some(m) = &self.month {
            output += format!("{:01$}", m, 2).as_str();
        }
        else {
            output += "??";
        }
        output += ".";
        if let Some(d) = &self.day {
            output += format!("{:01$}", d, 2).as_str();
        }
        else {
            output += "??"
        }
        write!(f, "{}", output)
    }
}

impl PgnDate {
    pub fn new(year: Option<i32>, month: Option<u8>, day: Option<u8>) -> PgnDate {
        PgnDate { year, month, day }
    }

    pub fn now() -> PgnDate {
        // Create a PGN date from the current datetime.
        let local = OffsetDateTime::now_local();
        match local {
            Ok(t) => {
                PgnDate {
                    year: Some(t.year()),
                    month: Some(t.month() as u8),
                    day: Some(t.day()),
                }
            }
            Err(_) => {
                PgnDate{ year: None, month: None, day: None }
            }
        }
    }
}

enum PgnResult {
    WhiteWin,
    BlackWin,
    Draw,
    Unknown
}

impl Display for PgnResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            PgnResult::WhiteWin => "1-0",
            PgnResult::BlackWin => "0-1",
            PgnResult::Draw => "1/2-1/2",
            PgnResult::Unknown => "*",
        };
        write!(f, "{}", output)
    }
}

enum PgnRound {
    Known(Vec<u32>),
    Unknown,
    Inappropriate,
}

impl Display for PgnRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut round_str = String::new();
        let output: &str = match self {
            PgnRound::Known(rounds) => {
                for i in 0..rounds.len() {
                    round_str += rounds[i].to_string().as_str();
                    if i != rounds.len() - 1 {
                        round_str += ".";
                    }
                }
                round_str.as_str()
            },
            PgnRound::Unknown => "?",
            PgnRound::Inappropriate => "-",
        };
        write!(f, "{}", output)
    }
}

impl PgnRound {
    pub fn from(round_str: &str) -> Result<PgnRound, ParseIntError> {
        let mut rounds: Vec<u32> = Vec::new();
        for round in round_str.trim().split(".") {
            let r = round.parse::<u32>()?;
            rounds.push(r);
        }
        Ok(PgnRound::Known(rounds))
    }
}

struct MoveList {
    moves: Vec<PgnMove>
}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let mut new_line = String::new();
        for i in 0..self.moves.len() {
            let mvs = format!("{}. {}", i + 1, self.moves[i].to_string());
            if mvs.len() + new_line.len() >= 80 {
                let mut carriage_returned = false;
                for token in mvs.split_whitespace() {
                    if carriage_returned {
                        if new_line.len() != 0 {
                            new_line += " ";
                        }
                        new_line += token;
                    }
                    else if token.len() + new_line.len() < 80 {
                        new_line += token;
                    }
                    else {
                        new_line += "\n";
                        carriage_returned = true;
                        output += new_line.as_str();
                        new_line.clear();
                    }
                }
            }
            else {
                new_line += mvs.as_str();
                new_line += " ";
            }
        }
        write!(f, "{}", output)
    }
}

struct PgnMove {
    white_move: Option<ChessMove>,
    black_move: Option<ChessMove>,
}

impl Display for PgnMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if let Some(wm) = &self.white_move {
            output += wm.to_string().as_str();
            output += " ";
            if let Some(bm) = &self.black_move {
                output += bm.to_string().as_str();
            }
        }
        write!(f, "{}", output)
    }
}

struct ChessMove {
    origin: Option<ChessCoordinate>,
    destination: Option<ChessCoordinate>,
    moving_piece: Option<ChessPiece>,
    castle: Option<ChessCastleDirection>,
    promotion: Option<ChessPiece>,
    is_capture: bool,
    is_check: bool,
    is_check_mate: bool,
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        if let Some(castle) = &self.castle {
            output += match castle {
                ChessCastleDirection::KingsideCastle => "O-O",
                ChessCastleDirection::QueensideCastle => "O-O-O",
            }
        }
        else {
            // Show piece
            if let Some(p) = &self.moving_piece {
                match p {
                    ChessPiece::Pawn => (), // pawn piece character is never shown.
                    _ => {
                        output += p.to_string().as_str();
                    }
                }
            }

            // Show origin
            if let Some(orig) = &self.origin {
                if let Some(f) = orig.get_file() {
                    output += f.to_string().as_str();
                }
                if let Some(r) = orig.get_rank() {
                    if let Some(p) = &self.moving_piece {
                        match p {
                            ChessPiece::Pawn => (), // pawn moves never need rank indication
                            _ => {
                                output += r.to_string().as_str();
                            }
                        }
                    }
                }
            }

            // Show capture
            if self.is_capture {
                output += "x"
            }

            // Show destination
            if let Some(dest) = &self.destination {
                if let Some(f) = dest.get_file() {
                    output += f.to_string().as_str();
                }
                if let Some(r) = dest.get_rank() {
                    output += r.to_string().as_str();
                }
            }

            // Show promotion
            if let Some(promote) = &self.promotion {
                output += promote.to_string().as_str();
            }
        }

        // Show check & check mate markers.
        if self.is_check_mate {
            output += "#"
        }
        else if self.is_check {
            output += "+"
        }

        write!(f, "{}", output)
    }
}

impl ChessMove {
    pub fn new() -> ChessMoveBuilder {
        ChessMoveBuilder::new()
    }

    pub fn from(pgn_move_string: String) -> Result<ChessMove, ChessMoveBuildError> {
        // Create move struct from an input string
        let new_move = ChessMove::new();
        todo!();
        new_move.build()
    }
}

struct ChessMoveBuilder {
    origin: Option<ChessCoordinate>,
    destination: Option<ChessCoordinate>,
    moving_piece: Option<ChessPiece>,
    castle: Option<ChessCastleDirection>,
    promotion: Option<ChessPiece>,
    is_capture: bool,
    is_check: bool,
    is_check_mate: bool,
}

enum ChessMoveBuildError {
    InvalidMove,
    ImpossibleMove,
    MissingDestination,
    MissingMoveData,
}

impl ChessMoveBuilder {
    pub fn new() -> ChessMoveBuilder {
        ChessMoveBuilder {
            origin: None,
            destination: None,
            moving_piece: None,
            castle: None,
            promotion: None,
            is_capture: false,
            is_check: false,
            is_check_mate: false,
        }
    }

    pub fn set_origin(mut self, origin: ChessCoordinate) -> ChessMoveBuilder {
        self.origin = Some(origin);
        self
    }

    pub fn set_destination(mut self, dest: ChessCoordinate) -> ChessMoveBuilder {
        self.destination = Some(dest);
        self
    }

    pub fn set_is_capture(mut self, capture: bool) -> ChessMoveBuilder {
        self.is_capture = capture;
        self
    }

    pub fn set_moving_piece(mut self, piece: ChessPiece) -> ChessMoveBuilder {
        self.moving_piece = Some(piece);
        self
    }

    pub fn set_castle(mut self, direction: ChessCastleDirection) -> ChessMoveBuilder {
        self.castle = Some(direction);
        self
    }

    pub fn set_promotion(mut self, piece: ChessPiece) -> ChessMoveBuilder {
        self.promotion = Some(piece);
        self
    }

    pub fn set_is_check(mut self, is_check: bool) -> ChessMoveBuilder {
        self.is_check = is_check;
        self
    }

    pub fn set_is_check_mate(mut self, is_mate: bool) -> ChessMoveBuilder {
        self.is_check_mate = is_mate;
        self
    }

    pub fn build(mut self) -> Result<ChessMove, ChessMoveBuildError> {
        // Verify a valid unambiguis move can be created from the given data.
        // Note: This does not check piece movement rules and only checks rules
        //       regarding PGN written notation rules.

        // Can't be check and check mate at the same time.
        if self.is_check && self.is_check_mate {
            return Err(ChessMoveBuildError::ImpossibleMove);
        }

        // If castling, can't be a capture or a promotion.
        if self.castle.is_some() && (self.is_capture || self.promotion.is_some()) {
            return Err(ChessMoveBuildError::ImpossibleMove);
        }

        // Destination must contain both rank and file.
        if let Some(dest) = &self.destination {
            if !dest.is_complete() {
                return Err(ChessMoveBuildError::MissingDestination);
            }
        }

        // If no destination given, must be a castling move.
        else if self.castle.is_none() {
            return Err(ChessMoveBuildError::MissingMoveData);
        }

        // If no piece is given, it is assumed to be a pawn.
        if self.moving_piece.is_none() {
            self.moving_piece = Some(ChessPiece::Pawn);
        }

        // Check piece specific rules
        if let Some(piece) = &self.moving_piece {
            match piece {
                ChessPiece::Pawn => {
                    if self.is_capture {
                        if let Some(orig) = &self.origin {
                            // If is a pawn capture, must contain the origin file.
                            if orig.get_file().is_none() {
                                return Err(ChessMoveBuildError::MissingMoveData);
                            }
                        }
                        else {
                            return Err(ChessMoveBuildError::MissingMoveData);
                        }
                    }
                },
                _ => ()
            }
        }
        Ok(ChessMove{
            origin: self.origin,
            destination: self.destination,
            moving_piece: self.moving_piece,
            castle: self.castle,
            promotion: self.promotion,
            is_capture: self.is_capture,
            is_check: self.is_check,
            is_check_mate: self.is_check_mate,
        })
    }
}

enum ChessPiece {
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

struct ChessCoordinate {
    rank: Option<ChessRank>,
    file: Option<ChessFile>,
}

impl ChessCoordinate {
    pub fn new(chess_rank: ChessRank, chess_file: ChessFile) -> ChessCoordinate {
        ChessCoordinate {
            rank: Some(chess_rank),
            file: Some(chess_file),
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

enum ChessRank {
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

enum ChessFile {
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

enum ChessCastleDirection {
    KingsideCastle,
    QueensideCastle,
}

// === UNIT TESTS ===

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_sample() {

    }
}