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

#[derive(Clone, Debug)]
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

    pub fn from(pgn_move_string: &str) -> Result<ChessMove, ChessMoveBuildError> {
        if pgn_move_string.len() == 0 {
            return Err(ChessMoveBuildError::MissingMoveData);
        }
        if !pgn_move_string.is_ascii() {
            return Err(ChessMoveBuildError::InvalidInputFormat);
        }
        let mov_str = pgn_move_string.trim();

        let mut new_move = ChessMove::new();

        // A local enum to help keep track of build loop phase while iterating through string.
        enum MoveBuildPhase {
            CheckCastle,
            PieceType,
            Origin,
            Capture,
            Destination,
            Promotion,
            Checks,
            Done,
        }

        // Example Moves:
        //  Nf6xd5
        //  O-O-O
        let mut phase = MoveBuildPhase::CheckCastle;
        let mut move_iter = mov_str.chars();
        let mut current_char = move_iter.next();
        let mut castle_count = 0;
        let mut board_file: Option<ChessFile> = None;
        let mut board_rank: Option<ChessRank> = None;
        loop {
            match phase {
                MoveBuildPhase::CheckCastle => {
                    let mut castle_finish = false;
                    if let Some(c) = current_char {
                        match c {
                            'O' => {
                                castle_count += 1
                            }
                            '-' => (),
                            _ => {
                                // Found a non castle char, resolve this phase.
                                castle_finish = true;
                            }
                        }
                    }
                    else {
                        castle_finish = true;
                    }

                    if castle_finish {
                        if castle_count == 3 {
                            new_move = new_move
                                .set_castle(ChessCastleDirection::QueensideCastle)
                                .set_moving_piece(ChessPiece::King);
                            phase = MoveBuildPhase::Checks;
                        }
                        else if castle_count == 2 {
                            new_move = new_move
                                .set_castle(ChessCastleDirection::KingsideCastle)
                                .set_moving_piece(ChessPiece::King);
                            phase = MoveBuildPhase::Checks;
                        }
                        else if castle_count == 0 && current_char.is_some() {
                            phase = MoveBuildPhase::PieceType;
                            continue; // let current char continue into next phase.
                        }
                        else {
                            return Err(ChessMoveBuildError::InvalidMove);
                        }
                    }
                },
                MoveBuildPhase::PieceType => {
                    if let Some(c) = current_char {
                        phase = MoveBuildPhase::Origin;
                        if let Some(p) = ChessPiece::from(c) {
                            new_move = new_move.set_moving_piece(p);
                        }
                        else {
                            continue; // let current char continue into next phase.
                        }
                    }
                    else {
                        return Err(ChessMoveBuildError::InvalidMove);
                    }
                },
                MoveBuildPhase::Origin|MoveBuildPhase::Destination => {
                    let mut complete = false;
                    if let Some(c) = current_char {
                        if let Some(f) = ChessFile::from(c) {
                            board_file = Some(f);
                        }
                        else if let Some(r) = ChessRank::from(c) {
                            board_rank = Some(r);
                        }
                        else {
                            complete = true;
                        }
                    }
                    else {
                        complete = true;
                    }

                    if complete {
                        let mut coord = ChessCoordinate::empty();
                        if let Some(f) = board_file {
                            coord.set_file(f);
                        }
                        if let Some(r) = board_rank {
                            coord.set_rank(r);
                        }
                        if !coord.is_empty() {
                            match phase {
                                MoveBuildPhase::Origin => {
                                    if current_char.is_some() {
                                        new_move = new_move.set_origin(coord)
                                    }
                                    else {
                                        // This is the only coordinate in the move string so it must be a destination square.
                                        new_move = new_move.set_destination(coord);
                                    }
                                },
                                MoveBuildPhase::Destination => new_move = new_move.set_destination(coord),
                                _ => (),
                            }
                        }
                        else {
                            return Err(ChessMoveBuildError::InvalidMove);
                        }
                        if current_char.is_none() {
                            // Reached the end of move the string.
                            break;
                        }
                        board_file = None;
                        board_rank = None;
                        phase = match phase {
                            MoveBuildPhase::Origin => MoveBuildPhase::Capture,
                            MoveBuildPhase::Destination =>MoveBuildPhase::Promotion,
                            _ => MoveBuildPhase::Done,
                        };
                        continue;
                    }
                },
                MoveBuildPhase::Capture => {
                    if let Some(c) = current_char {
                        phase = MoveBuildPhase::Destination;
                        if c == 'x' {
                            new_move = new_move.set_is_capture(true);
                        }
                        else {
                            continue;
                        }
                    }
                    else {
                        return Err(ChessMoveBuildError::InvalidMove);
                    }
                },
                MoveBuildPhase::Promotion => {
                    if let Some(c) = current_char {
                        phase = MoveBuildPhase::Checks;
                        if c == '=' {
                            // is promotion, iterate to the next char and get promotion piece.
                            current_char = move_iter.next();
                            if let Some(c) = current_char {
                                if let Some(p) = ChessPiece::from(c) {
                                    new_move = new_move.set_promotion(p);
                                }
                                else {
                                    return Err(ChessMoveBuildError::InvalidMove);
                                }
                            }
                            else {
                                return Err(ChessMoveBuildError::InvalidMove);
                            }
                        }
                    }
                    else {
                        break;
                    }
                }
                MoveBuildPhase::Checks => {
                    if let Some(c) = current_char {
                        phase = MoveBuildPhase::Done;
                        if c == '+' {
                            new_move = new_move.set_is_check(true);
                        }
                        else if c == '#' {
                            new_move = new_move.set_is_check_mate(true);
                        }
                        else {
                            return Err(ChessMoveBuildError::InvalidMove);
                        }
                    }
                    else {
                        break;
                    }
                },
                MoveBuildPhase::Done => break,
            }
            current_char = move_iter.next();
        }
        new_move.build()
    }

    pub fn get_origin(&self) -> Option<&ChessCoordinate> {
        if let Some(o) = &self.origin {
            return Some(&o);
        }
        None
    }

    pub fn get_destination(&self) -> Option<&ChessCoordinate> {
        if let Some(d) = &self.destination {
            return Some(&d);
        }
        None
    }

    pub fn get_moving_piece(&self) -> Option<&ChessPiece> {
        if let Some(mp) = &self.moving_piece {
            return Some(&mp);
        }
        None
    }

    pub fn get_castle(&self) -> Option<&ChessCastleDirection> {
        if let Some(c) = &self.castle {
            return Some(&c);
        }
        None
    }

    pub fn get_promotion(&self) -> Option<&ChessPiece> {
        if let Some(p) = &self.promotion {
            return Some(&p);
        }
        None
    }

    pub fn is_capture(&self) -> bool {
        self.is_capture
    }

    pub fn is_check(&self) -> bool {
        self.is_check
    }

    pub fn is_check_mate(&self) -> bool {
        self.is_check_mate
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

#[derive(Debug, PartialEq)]
enum ChessMoveBuildError {
    InvalidMove,
    ImpossibleMove,
    MissingDestination,
    MissingMoveData,
    InvalidInputFormat,
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
struct ChessCoordinate {
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ChessCastleDirection {
    KingsideCastle,
    QueensideCastle,
}

// === UNIT TESTS ===

#[cfg(test)]
mod test_move_parsing {

    use crate::chess_pgn::{ChessMoveBuildError, ChessFile, ChessPiece};

    use super::{ChessMove, ChessCoordinate, ChessRank, ChessCastleDirection};

    #[derive(Debug)]
    enum ExpectedParameter {
        ExpectOrigin(Option<ChessCoordinate>),
        ExpectDestination(Option<ChessCoordinate>),
        ExpectMovingPiece(Option<ChessPiece>),
        ExpectCastle(Option<ChessCastleDirection>),
        ExpectPromotion(Option<ChessPiece>),
        ExpectCapture(bool),
        ExpectCheck(bool),
        ExpectCheckMate(bool),
        ExpectError(ChessMoveBuildError),
    }

    fn test_move_parser_helper(test_str: &str, params: Vec<ExpectedParameter>) {
        let m_result = ChessMove::from(test_str);

        let mut tested_origin = false;
        let mut tested_destination = false;
        let mut tested_moving_piece = false;
        let mut tested_castle = false;
        let mut tested_promotion = false;
        let mut tested_capture = false;
        let mut tested_check = false;
        let mut tested_check_mate = false;

        match m_result {
            Ok(mov) => {
                for param in params {
                    match param {
                        ExpectedParameter::ExpectOrigin(expected_value) => {
                            tested_origin = true;
                            if let Some(expected) = expected_value {
                                assert!(mov.get_origin().is_some());
                                if let Some(result_value) = mov.get_origin() {
                                    assert_eq!(*result_value, expected);
                                }
                            }
                            else {
                                assert!(mov.get_origin().is_none());
                            }
                        },
                        ExpectedParameter::ExpectDestination(expected_value) => {
                            tested_destination = true;
                            if let Some(expected) = expected_value {
                                assert!(mov.get_destination().is_some());
                                if let Some(result_value) = mov.get_destination() {
                                    assert_eq!(*result_value, expected);
                                }
                            }
                            else {
                                assert!(mov.get_destination().is_none());
                            }
                        },
                        ExpectedParameter::ExpectMovingPiece(expected_value) => {
                            tested_moving_piece = true;
                            if let Some(expected) = expected_value {
                                assert!(mov.get_moving_piece().is_some());
                                if let Some(result_value) = mov.get_moving_piece() {
                                    assert_eq!(*result_value, expected);
                                }
                            }
                            else {
                                assert!(mov.get_moving_piece().is_none());
                            }
                        },
                        ExpectedParameter::ExpectCastle(expected_value) => {
                            tested_castle = true;
                            if let Some(expected) = expected_value {
                                assert!(mov.get_castle().is_some());
                                if let Some(result_value) = mov.get_castle() {
                                    assert_eq!(*result_value, expected);
                                }
                            }
                            else {
                                assert!(mov.get_castle().is_none());
                            }
                        },
                        ExpectedParameter::ExpectPromotion(expected_value) => {
                            tested_promotion = true;
                            if let Some(expected) = expected_value {
                                assert!(mov.get_promotion().is_some());
                                if let Some(result_value) = mov.get_promotion() {
                                    assert_eq!(*result_value, expected);
                                }
                            }
                            else {
                                assert!(mov.get_promotion().is_none());
                            }
                        },
                        ExpectedParameter::ExpectCapture(expected_value) => {
                            tested_capture = true;
                            assert_eq!(mov.is_capture(), expected_value);
                        },
                        ExpectedParameter::ExpectCheck(expected_value) => {
                            tested_check = true;
                            assert_eq!(mov.is_check(), expected_value);
                        },
                        ExpectedParameter::ExpectCheckMate(expected_value) => {
                            tested_check_mate = true;
                            assert_eq!(mov.is_check_mate(), expected_value);
                        },
                        ExpectedParameter::ExpectError(e) => {
                            panic!("Testing for error {:?} in string \"{:?}\" but error was not encountered.", e, test_str);
                        },
                    }
                }

                if !tested_origin {
                    assert!(mov.get_origin().is_none());
                }
                if !tested_destination {
                    assert!(mov.get_destination().is_none());
                }
                if !tested_moving_piece {
                    assert!(mov.get_moving_piece().is_none());
                }
                if !tested_castle {
                    assert!(mov.get_castle().is_none());
                }
                if !tested_promotion {
                    assert!(mov.get_promotion().is_none())
                };
                if !tested_capture {
                    assert_eq!(mov.is_capture(), false);
                }
                if !tested_check {
                    assert_eq!(mov.is_check(), false);
                }
                if !tested_check_mate {
                    assert_eq!(mov.is_check_mate(), false);
                }
            }
            Err(resulting_error) => {
                for param in params {
                    match param {
                        ExpectedParameter::ExpectError(expected_error) => {
                            assert_eq!(resulting_error, expected_error);
                        }
                        _ => {
                            panic!("Testing for successful move parse parameter {:?} in string \"{:?}\" but an error was encountered: {:?}", param, test_str, resulting_error);
                        }
                    }
                }
            }
        }
    }

    #[test]
    pub fn test_move_parsing_empty_fails() {
        test_move_parser_helper("", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::MissingMoveData),
        ]);
    }

    #[test]
    pub fn test_move_parsing_nonsense_fails() {
        test_move_parser_helper("asdf;lkj", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);
    }

    #[test]
    pub fn test_move_parsing_non_ascii_string_fails() {
        test_move_parser_helper("🤔", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidInputFormat),
        ]);
    }

    #[test]
    pub fn test_move_parsing_kingside_castle_passes() {
        test_move_parser_helper("O-O", vec![
            ExpectedParameter::ExpectCastle(Some(ChessCastleDirection::KingsideCastle)),
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::King)),
        ]);
    }

    #[test]
    pub fn test_move_parsing_queenside_castle_passes() {
        test_move_parser_helper("O-O-O", vec![
            ExpectedParameter::ExpectCastle(Some(ChessCastleDirection::QueensideCastle)),
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::King)),
        ]);
    }

    #[test]
    pub fn test_move_parsing_invalid_castles_fails() {

        test_move_parser_helper("O", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);

        test_move_parser_helper("O-O-O-O", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);

        test_move_parser_helper("O-", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);
    }

    #[test]
    pub fn test_move_parsing_simple_move_passes() {
        test_move_parser_helper("e4", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::Pawn)),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { file: Some(ChessFile::E), rank: Some(ChessRank::R4) })),
        ]);

        test_move_parser_helper("Nc3", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::Knight)),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { rank: Some(ChessRank::R3), file: Some(ChessFile::C) })),
        ]);

        test_move_parser_helper("Bf4", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::Bishop)),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { rank: Some(ChessRank::R4), file: Some(ChessFile::F) })),
        ]);

        test_move_parser_helper("Rb1", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::Rook)),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { rank: Some(ChessRank::R1), file: Some(ChessFile::B) })),
        ]);

        test_move_parser_helper("Qd3", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::Queen)),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { rank: Some(ChessRank::R3), file: Some(ChessFile::D) })),
        ]);
        test_move_parser_helper("Kf1", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::King)),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { rank: Some(ChessRank::R1), file: Some(ChessFile::F) })),
        ]);
    }

    #[test]
    pub fn test_move_parsing_simple_move_fails() {
        test_move_parser_helper("Pe4", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);

        test_move_parser_helper("Bk4", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);

        test_move_parser_helper("BF0", vec![
            ExpectedParameter::ExpectError(ChessMoveBuildError::InvalidMove),
        ]);
    }

    #[test]
    pub fn test_move_parsing_simple_capture_passes() {
        test_move_parser_helper("exd5", vec![
            ExpectedParameter::ExpectMovingPiece(Some(ChessPiece::Pawn)),
            ExpectedParameter::ExpectOrigin(Some(ChessCoordinate { file: Some(ChessFile::E), rank: None })),
            ExpectedParameter::ExpectDestination(Some(ChessCoordinate { file: Some(ChessFile::D), rank: Some(ChessRank::R5) })),
            ExpectedParameter::ExpectCapture(true),
        ]);
    }
}