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
            output += self.result.get_value().to_string().as_str();
        }

        write!(f, "{}", output)
    }
}

impl PgnGame {
    pub fn new() -> PgnGame {
        PgnGame {
            event: PgnTagPair::new(String::from("Event"), String::new()),
            site: PgnTagPair::new(String::from("Site"), String::new()),
            date: PgnTagPair::new(String::from("Date"), PgnDate::now()),
            round: PgnTagPair::new(String::from("Round"), PgnRound::Unknown),
            white: PgnTagPair::new(String::from("White"), String::new()),
            black: PgnTagPair::new(String::from("Black"), String::new()),
            result: PgnTagPair::new(String::from("Result"), PgnResult::Unknown),
            moves: MoveList::new(),
        }
    }

    pub fn set_event(&mut self, event: String) {
        self.event.set_value(event);
    }

    pub fn get_event(&self) -> &String {
        self.event.get_value()
    }

    pub fn set_site(&mut self, site: String) {
        self.site.set_value(site);
    }

    pub fn get_site(&self) -> &String {
        self.site.get_value()
    }

    pub fn set_date(&mut self, date: PgnDate) {
        self.date.set_value(date);
    }

    pub fn get_date(&self) -> &PgnDate {
        self.date.get_value()
    }

    pub fn set_round(&mut self, round: PgnRound) {
        self.round.set_value(round);
    }

    pub fn get_round(&self) -> &PgnRound {
        self.round.get_value()
    }

    pub fn set_white(&mut self, white: String) {
        self.white.set_value(white);
    }

    pub fn get_white(&self) -> &String {
        self.white.get_value()
    }

    pub fn set_black(&mut self, black: String) {
        self.black.set_value(black);
    }

    pub fn get_black(&self) -> &String {
        self.black.get_value()
    }

    pub fn set_result(&mut self, result: PgnResult) {
        self.result.set_value(result);
    }

    pub fn get_result(&self) -> &PgnResult {
        self.result.get_value()
    }

    pub fn push_move(&mut self, new_move: ChessMove) {
        self.moves.push_move(new_move);
    }

    pub fn pop_move(&mut self) -> Option<ChessMove> {
        self.moves.pop_move()
    }

    pub fn get_turn(&self) -> ChessTurn {
        self.moves.get_turn()
    }
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

impl<T: Display> PgnTagPair<T> {
    pub fn new(tag_name: String, tag_value: T) -> PgnTagPair<T> {
        PgnTagPair { tag_name, tag_value, }
    }

    pub fn set_value(&mut self, tag_value: T) {
        self.tag_value = tag_value;
    }

    pub fn get_value(&self) -> &T {
        &self.tag_value
    }

    pub fn get_name(&self) -> &String {
        &self.tag_name
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
        output += new_line.as_str();
        write!(f, "{}", output)
    }
}

enum ChessTurn {
    WhiteToMove,
    BlackToMove,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: Vec::new(),
        }
    }

    pub fn push_move(&mut self, new_move: ChessMove) {
        if self.moves.is_empty() {
            self.moves.push(PgnMove::new())
        }

        if let Some(m) = self.moves.last_mut() {
            match m.get_state() {
                PgnMoveState::MoveComplete => {
                    let mut new_pgn_move = PgnMove::new();
                    new_pgn_move.add_move(new_move);
                    self.moves.push(new_pgn_move);
                }
                _ => {
                    m.add_move(new_move);
                }
            }
        }
    }

    pub fn pop_move(&mut self) -> Option<ChessMove> {
        let mut ret_move = None;

        while ret_move.is_none() && self.moves.len() > 0 {
            let index = self.moves.len() - 1;
            let pgn_move = &mut self.moves[index];
            ret_move = pgn_move.remove_move();
            if ret_move.is_none() {
                self.moves.pop();
            }
        }

        return ret_move;
    }

    pub fn get_turn(&self) -> ChessTurn {
        if let Some(m) = self.moves.last() {
            let state = m.get_state();
            match state {
                PgnMoveState::MoveComplete => {
                    return ChessTurn::WhiteToMove;
                }
                PgnMoveState::WhiteToMove => {
                    return ChessTurn::WhiteToMove;
                }
                PgnMoveState::BlackToMove => {
                    return ChessTurn::BlackToMove;
                }
            }
        }
        return ChessTurn::WhiteToMove;
    }
}

enum PgnMoveState {
    WhiteToMove,
    BlackToMove,
    MoveComplete,
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

impl PgnMove {
    pub fn new() -> PgnMove {
        PgnMove { white_move: None, black_move: None }
    }

    pub fn get_state(&self) -> PgnMoveState {
        if self.white_move.is_none() {
            return PgnMoveState::WhiteToMove;
        }
        else if self.black_move.is_none() {
            return PgnMoveState::BlackToMove;
        }
        else {
            return PgnMoveState::MoveComplete;
        }
    }

    pub fn add_move(&mut self, new_move: ChessMove) -> bool {
        if self.white_move.is_none() {
            self.white_move = Some(new_move);
        }
        else if self.black_move.is_none() {
            self.black_move = Some(new_move);
        }
        else {
            return false;
        }
        return true;
    }

    pub fn remove_move(&mut self) -> Option<ChessMove> {
        let mut temp: Option<ChessMove> = None;
        if let Some(m) = &self.black_move {
            temp = Some(m.clone());
            self.black_move = None;
        }
        else if let Some(m) = &self.white_move {
            temp = Some(m.clone());
            self.white_move = None;
        }
        return temp;
    }
}

#[derive(Clone)]
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

#[derive(Debug)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum ChessCastleDirection {
    KingsideCastle,
    QueensideCastle,
}

// === UNIT TESTS ===

#[cfg(test)]
mod tests {
    use super::{PgnGame, ChessMove, ChessCoordinate, ChessRank};

    #[test]
    pub fn test_sample() {
        let mut pgn = PgnGame::new();
        let m = ChessMove::new()
            .set_destination(ChessCoordinate::new(ChessRank::R4, super::ChessFile::E))
            .build().unwrap();
        pgn.push_move(m);
        println!("{pgn}")
    }
}