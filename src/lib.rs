
//! Module for parsing standard algebraic notation in chess.
//! Supports parsing SAN strings into usable data structures, 
//! as well as converting the data structures back to string.

use regex::Regex;

macro_rules! pos_col {
    ($cap:expr, $col:expr) => {
        Some($cap[$col].chars().next().unwrap() as usize - 0x61)
    };
}

macro_rules! pos_row {
    ($cap:expr, $row:expr) => {
        Some(7 - ($cap[$row].parse::<usize>().unwrap() - 1))
    };
}

macro_rules! pos {
    ($cap:expr, $col:expr, $row:expr) => {
        Position::new(pos_col!($cap, $col), pos_row!($cap, $row))
    };
}

macro_rules! check_type {
    ($cap:expr, $i:expr) => {
        {
            let val = $cap.get($i).map_or("", |v| v.as_str());
            if val == "+" {
                Some(CheckType::Check);
            }
            else if val == "#" {
                Some(CheckType::Mate);
            }
            None
        }
    };
}

macro_rules! annotation {
    ($cap:expr, $i:expr) => {
        Annotation::from_str($cap.get($i).map_or("", |v| v.as_str())).ok()
    };
}

macro_rules! promotion {
    ($cap:expr, $i:expr) => {
        Piece::from_str($cap.get($i).map_or("fail", |v| v.as_str())).ok();
    };
}

/**
 * Represents a completely unspecified position.
 */
pub const POS_NONE: Position = Position {x: None, y: None};

#[derive(Debug, Eq, PartialEq)]
pub enum SanError {
    IllegalInput(String),
    RegexExhausted(String),
}

impl std::error::Error for SanError {}

pub type Result<T> = std::result::Result<T, SanError>;

/**
 * Methods for converting between internal and string representations.
 */
trait StrEnum {
    type Output;
    fn to_str(&self) -> &str;
    fn from_str(value: &str) -> Result<Self::Output>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Piece {
    Pawn,
    Bishop,
    King,
    Knight,
    Queen,
    Rook
}

impl StrEnum for Piece {
    type Output = Piece;

    fn to_str(&self) -> &str {
        match self {
            Piece::Pawn => "",
            Piece::Bishop => "B",
            Piece::King => "K",
            Piece::Knight => "N",
            Piece::Queen => "Q",
            Piece::Rook => "R"
        }
    }

    fn from_str(value: &str) -> Result<Piece> {
        match value {
            "" => Ok(Piece::Pawn),
            "B" => Ok(Piece::Bishop),
            "K" => Ok(Piece::King),
            "N" => Ok(Piece::Knight),
            "Q" => Ok(Piece::Queen),
            "R" => Ok(Piece::Rook),
            _ => Err(SanError::IllegalInput(format!("invalid piece: {}", value)))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Annotation {
    Blunder,
    Mistake,
    Interesting,
    Good,
    Brilliant
}

impl StrEnum for Annotation {
    type Output = Annotation;

    fn to_str(&self) -> &str {
        match self {
            Annotation::Blunder => "??",
            Annotation::Mistake => "?",
            Annotation::Interesting => "?!",
            Annotation::Good => "!",
            Annotation::Brilliant => "!!"
        }
    }

    fn from_str(value: &str) -> Result<Annotation> {
        match value {
            "??" => Ok(Annotation::Blunder),
            "?" => Ok(Annotation::Mistake),
            "?!" => Ok(Annotation::Interesting),
            "!" => Ok(Annotation::Good),
            "!!" => Ok(Annotation::Brilliant),
            _ => Err(SanError::IllegalInput(format!("invalid annotation: {}", value)))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CastleType {
    Kingside,
    Queenside
}

impl StrEnum for CastleType {
    type Output = CastleType;

    fn to_str(&self) -> &str {
        match self {
            CastleType::Kingside => "O-O",
            CastleType::Queenside => "O-O-O"
        }
    }

    fn from_str(value: &str) -> Result<CastleType> {
        match value {
            "O-O" => Ok(CastleType::Kingside),
            "O-O-O" => Ok(CastleType::Queenside),
            _ => Err(SanError::IllegalInput(format!("invalid castling move: {}", value)))
        }
    }
}

/**
 * Represents a square on the board.
 * x -> file,
 * y -> rank.
 */
#[derive(Debug, Eq, PartialEq)]
pub struct Position {
    pub x: Option<usize>,
    pub y: Option<usize>
}

impl Position {
    pub fn new(x: Option<usize>, y: Option<usize>) -> Position {
        Position { x, y }
    }

    pub fn of(x: usize, y: usize) -> Position {
        Position {
            x: Some(x),
            y: Some(y)
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        let mut res = String::new();
        if let Some(x) = self.x {
            res.push(char::from(b'a' + (x as u8)));
        }
        if let Some(y) = self.y {
            res.push(char::from(b'8' - (y as u8)));
        }
        res
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MoveKind {
    /**
     * Order: (origin, destination)
     */
    Normal(Position, Position),
    Castle(CastleType)
}

#[derive(Debug, Eq, PartialEq)]
pub enum CheckType {
    Check,
    Mate
}

/**
 * Data structure representing a single move.
 * 
 * The coordinates are defined with the origin (0,0) as the top left corner,
 * and (7,7) as the bottom right corner, with white pieces in bottom rows.
 */
#[derive(Debug)]
pub struct Move {
    pub move_kind: MoveKind,
    pub piece: Piece,
    pub promotion: Option<Piece>,
    pub annotation: Option<Annotation>,
    pub check_type: Option<CheckType>,
    pub is_capture: bool
}

impl Move {
    pub fn new(piece: Piece, move_kind: MoveKind) -> Move {
        Move {
            move_kind,
            piece,
            promotion: None,
            annotation: None,
            check_type: None,
            is_capture: false
        }
    }

    /**
     * Compiles the data in a Move struct into a SAN string.
     */
    pub fn compile(&self) -> String {
        let mut res = String::new();

        match &self.move_kind {
            MoveKind::Castle(t) => res.push_str(t.to_str()),
            MoveKind::Normal(src, dst) => {
                res.push_str(self.piece.to_str());
                res.push_str(&src.to_string());
                if self.is_capture {
                    res.push('x');
                }
                res.push_str(&dst.to_string());
            }
        }
        

        if let Some(piece) = &self.promotion {
            res.push('=');
            res.push_str(piece.to_str());
        }

        if let Some(ct) = &self.check_type {
            match ct {
                CheckType::Check => res.push('+'),
                CheckType::Mate => res.push('#')
            }
        }

        if let Some(ann) = &self.annotation {
            res.push_str(ann.to_str());
        }

        return res;
    }

    /**
     * Parses a SAN string and creates a Move data struct.
     */
    pub fn parse(value: &str) -> Result<Move> {

        // Check for castling:
        let re = Regex::new(r"^(O-O|O-O-O)(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::King, 
                MoveKind::Castle(CastleType::from_str(&cap[1]) ?));
            mov.check_type = check_type!(cap, 2);
            mov.annotation = annotation!(cap, 3);
            return Ok(mov);
        }

        // Pawn movement:
        let re = Regex::new(r"^([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::Pawn,
                MoveKind::Normal(POS_NONE, pos!(cap, 1, 2)));
            mov.check_type = check_type!(cap, 3);
            mov.annotation = annotation!(cap, 4);
            return Ok(mov);
        }

        // Pawn movement (long san):
        let re = Regex::new(r"^([a-h])([1-8])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::Pawn,
                MoveKind::Normal(pos!(cap, 1, 2), pos!(cap, 3, 4)));
            mov.check_type = check_type!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece movement:
        let re = Regex::new(r"^([KQBNR])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(POS_NONE, pos!(cap, 2, 3)));
            mov.check_type = check_type!(cap, 4);
            mov.annotation = annotation!(cap, 5);
            return Ok(mov);
        }

        // Piece movement from a specific column:
        let re = Regex::new(r"^([KQBNR])([a-h])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(Position::new(pos_col!(cap, 2), None), pos!(cap, 3, 4)));
            mov.check_type = check_type!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece movement from a specific row:
        let re = Regex::new(r"^([KQBNR])([0-9])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(Position::new(None, pos_row!(cap, 2)), pos!(cap, 3, 4)));
            mov.check_type = check_type!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece movement from a specific column and row (long san):
        let re = Regex::new(r"^([KQBNR])([a-h])([0-9])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(pos!(cap, 2, 3), pos!(cap, 4, 5)));
            mov.check_type = check_type!(cap, 6);
            mov.annotation = annotation!(cap, 7);
            return Ok(mov);
        }

        // Pawn capture:
        let re = Regex::new(r"^([a-h])x([a-h])([1-8])(?:=?([KQBNR]))?(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::Pawn, 
                MoveKind::Normal(Position::new(pos_col!(cap, 1), None), pos!(cap, 2, 3)));
            mov.is_capture = true;
            mov.promotion = promotion!(cap, 4);
            mov.check_type = check_type!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Pawn capture (long san):
        let re = Regex::new(r"^([a-h])([1-8])x([a-h])([1-8])(?:=?([KQBNR]))?(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::Pawn, 
                MoveKind::Normal(pos!(cap, 1, 2), pos!(cap, 3, 4)));
            mov.is_capture = true;
            mov.promotion = promotion!(cap, 5);
            mov.check_type = check_type!(cap, 6);
            mov.annotation = annotation!(cap, 7);
            return Ok(mov);
        }

        // Piece capture:
        let re = Regex::new(r"^([KQBNR])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(POS_NONE, pos!(cap, 2, 3)));
            mov.is_capture = true;
            mov.check_type = check_type!(cap, 4);
            mov.annotation = annotation!(cap, 5);
            return Ok(mov);
        }

        // Piece capture from a specific column:
        let re = Regex::new(r"^([KQBNR])([a-h])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(Position::new(pos_col!(cap, 2), None), pos!(cap, 3, 4)));
            mov.is_capture = true;
            mov.check_type = check_type!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece capture from a specific row:
        let re = Regex::new(r"^([KQBNR])([0-9])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(Position::new(None, pos_row!(cap, 2)), pos!(cap, 3, 4)));
            mov.is_capture = true;
            mov.check_type = check_type!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece capture from a specific column and row (long san):
        let re = Regex::new(r"^([KQBNR])([a-h])([0-9])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::from_str(&cap[1]) ?, 
                MoveKind::Normal(pos!(cap, 2, 3), pos!(cap, 4, 5)));
            mov.is_capture = true;
            mov.check_type = check_type!(cap, 6);
            mov.annotation = annotation!(cap, 7);
            return Ok(mov);
        }

        // Check for pawn promotion:
        let re = Regex::new(r"^([a-h])([1-8])=?([KQBNR])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            let mut mov = Move::new(
                Piece::Pawn,
                MoveKind::Normal(POS_NONE, pos!(cap, 1, 2)));
            mov.promotion = promotion!(cap, 3);
            mov.check_type = check_type!(cap, 4);
            mov.annotation = annotation!(cap, 5);
            return Ok(mov);
        }

        Err(SanError::RegexExhausted(format!("could not parse: {}", value)))
    }
}

#[cfg(test)]
mod tests;
