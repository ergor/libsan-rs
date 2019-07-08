
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

macro_rules! is_check {
    ($cap:expr, $i:expr) => {
        $cap.get($i).map_or("", |v| v.as_str()) == "+"
    };
}

macro_rules! is_check_mate {
    ($cap:expr, $i:expr) => {
        $cap.get($i).map_or("", |v| v.as_str()) == "#"
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

pub const POS_NONE: Position = Position {x: None, y: None};

trait StrEnum {
    type Output;
    fn to_str(&self) -> &str;
    fn from_str(value: &str) -> Result<Self::Output, &str>;
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

    fn from_str(value: &str) -> Result<Piece, &str> {
        match value {
            "" => Ok(Piece::Pawn),
            "B" => Ok(Piece::Bishop),
            "K" => Ok(Piece::King),
            "N" => Ok(Piece::Knight),
            "Q" => Ok(Piece::Queen),
            "R" => Ok(Piece::Rook),
            _ => Err("no such piece")
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

    fn from_str(value: &str) -> Result<Annotation, &str> {
        match value {
            "??" => Ok(Annotation::Blunder),
            "?" => Ok(Annotation::Mistake),
            "?!" => Ok(Annotation::Interesting),
            "!" => Ok(Annotation::Good),
            "!!" => Ok(Annotation::Brilliant),
            _ => Err("not an annotation")
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

    fn from_str(value: &str) -> Result<CastleType, &str> {
        match value {
            "O-O" => Ok(CastleType::Kingside),
            "O-O-O" => Ok(CastleType::Queenside),
            _ => Err("not a castling move")
        }
    }
}

/**
 * Defined as (x,y):
 * 0,0 ... 7,0
 *  .  \    .
 *  .    \  .
 * 0,7 ... 7,7
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
pub enum MoveType {
    /**
     * Order: (origin, destination)
     */
    Normal(Position, Position),
    Castle(CastleType),
    Undefined
}

#[derive(Debug)]
pub struct Move {
    pub move_type: MoveType,
    pub piece: Option<Piece>,
    pub promotion: Option<Piece>,
    pub annotation: Option<Annotation>,
    pub is_capture: bool,
    pub is_check: bool,
    pub is_check_mate: bool,
}

impl ToString for Move {
    fn to_string(&self) -> String {
        let mut res = String::new();
        match &self.move_type {
            MoveType::Castle(t) => res.push_str(t.to_str()),
            MoveType::Normal(src, dst) => {
                match &self.piece {
                    Some(p) => res.push_str(p.to_str()),
                    None => panic!("Move: expected Some(Piece), got None")
                }
                res.push_str(&src.to_string());
                if self.is_capture {
                    res.push('x');
                }
                res.push_str(&dst.to_string());
            }
            _ => panic!("MoveType not defined")
        }
        if let Some(piece) = &self.promotion {
            res.push('=');
            res.push_str(piece.to_str());
        }
        if self.is_check {
            res.push('+');
        }
        else if self.is_check_mate {
            res.push('#');
        }
        if let Some(ann) = &self.annotation {
            res.push_str(ann.to_str());
        }
        return res;
    }
}

impl Move {
    pub fn new() -> Move {
        Move {
            move_type: MoveType::Undefined,
            piece: None,
            promotion: None,
            annotation: None,
            is_capture: false,
            is_check: false,
            is_check_mate: false
        }
    }

    pub fn parse(value: &str) -> Result<Move, String> {
        let mut mov = Move::new();

        // Check for castling:
        let re = Regex::new(r"^(O-O|O-O-O)(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.move_type = MoveType::Castle(CastleType::from_str(&cap[1]).unwrap());
            mov.is_check = is_check!(cap, 2);
            mov.is_check_mate = is_check_mate!(cap, 2);
            mov.annotation = annotation!(cap, 3);
            return Ok(mov);
        }

        // Pawn movement:
        let re = Regex::new(r"^([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Some(Piece::Pawn);
            mov.move_type = MoveType::Normal(POS_NONE, pos!(cap, 1, 2));
            mov.is_check = is_check!(cap, 3);
            mov.is_check_mate = is_check_mate!(cap, 3);
            mov.annotation = annotation!(cap, 4);
            return Ok(mov);
        }

        // Pawn movement (long san):
        let re = Regex::new(r"^([a-h])([1-8])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Some(Piece::Pawn);
            mov.move_type = MoveType::Normal(pos!(cap, 1, 2), pos!(cap, 3, 4));
            mov.is_check = is_check!(cap, 5);
            mov.is_check_mate = is_check_mate!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece movement:
        let re = Regex::new(r"^([KQBNR])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(POS_NONE, pos!(cap, 2, 3));
            mov.is_check = is_check!(cap, 4);
            mov.is_check_mate = is_check_mate!(cap, 4);
            mov.annotation = annotation!(cap, 5);
            return Ok(mov);
        }

        // Piece movement from a specific column:
        let re = Regex::new(r"^([KQBNR])([a-h])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(Position::new(pos_col!(cap, 2), None), pos!(cap, 3, 4));
            mov.is_check = is_check!(cap, 5);
            mov.is_check_mate = is_check_mate!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece capture from a specific row:
        let re = Regex::new(r"^([KQBNR])([0-9])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(Position::new(None, pos_row!(cap, 2)), pos!(cap, 3, 4));
            mov.is_check = is_check!(cap, 5);
            mov.is_check_mate = is_check_mate!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece movement from a specific column and row (long san):
        let re = Regex::new(r"^([KQBNR])([a-h])([0-9])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(pos!(cap, 2, 3), pos!(cap, 4, 5));
            mov.is_check = is_check!(cap, 6);
            mov.is_check_mate = is_check_mate!(cap, 6);
            mov.annotation = annotation!(cap, 7);
            return Ok(mov);
        }

        // Pawn capture:
        let re = Regex::new(r"^([a-h])x([a-h])([1-8])(?:=?([KQBNR]))?(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Some(Piece::Pawn);
            mov.move_type = MoveType::Normal(Position::new(pos_col!(cap, 1), None), pos!(cap, 2, 3));
            mov.is_capture = true;
            mov.promotion = promotion!(cap, 4);
            mov.is_check = is_check!(cap, 5);
            mov.is_check_mate = is_check_mate!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Pawn capture (long san):
        let re = Regex::new(r"^([a-h])([1-8])x([a-h])([1-8])(?:=?([KQBNR]))?(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Some(Piece::Pawn);
            mov.move_type = MoveType::Normal(pos!(cap, 1, 2), pos!(cap, 3, 4));
            mov.is_capture = true;
            mov.promotion = promotion!(cap, 5);
            mov.is_check = is_check!(cap, 6);
            mov.is_check_mate = is_check_mate!(cap, 6);
            mov.annotation = annotation!(cap, 7);
            return Ok(mov);
        }

        // Piece capture:
        let re = Regex::new(r"^([KQBNR])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(POS_NONE, pos!(cap, 2, 3));
            mov.is_capture = true;
            mov.is_check = is_check!(cap, 4);
            mov.is_check_mate = is_check_mate!(cap, 4);
            mov.annotation = annotation!(cap, 5);
            return Ok(mov);
        }

        // Piece capture from a specific column:
        let re = Regex::new(r"^([KQBNR])([a-h])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(Position::new(pos_col!(cap, 2), None), pos!(cap, 3, 4));
            mov.is_capture = true;
            mov.is_check = is_check!(cap, 5);
            mov.is_check_mate = is_check_mate!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece capture from a specific row:
        let re = Regex::new(r"^([KQBNR])([0-9])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(Position::new(None, pos_row!(cap, 2)), pos!(cap, 3, 4));
            mov.is_capture = true;
            mov.is_check = is_check!(cap, 5);
            mov.is_check_mate = is_check_mate!(cap, 5);
            mov.annotation = annotation!(cap, 6);
            return Ok(mov);
        }

        // Piece capture from a specific column and row (long san):
        let re = Regex::new(r"^([KQBNR])([a-h])([0-9])x([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Piece::from_str(&cap[1]).ok();
            mov.move_type = MoveType::Normal(pos!(cap, 2, 3), pos!(cap, 4, 5));
            mov.is_capture = true;
            mov.is_check = is_check!(cap, 6);
            mov.is_check_mate = is_check_mate!(cap, 6);
            mov.annotation = annotation!(cap, 7);
            return Ok(mov);
        }

        // Check for pawn promotion:
        let re = Regex::new(r"^([a-h])([1-8])=?([KQBNR])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
        if re.is_match(value) {
            let cap = re.captures(value).unwrap();
            mov.piece = Some(Piece::Pawn);
            mov.move_type = MoveType::Normal(POS_NONE, pos!(cap, 1, 2));
            mov.promotion = promotion!(cap, 3);
            mov.is_check = is_check!(cap, 4);
            mov.is_check_mate = is_check_mate!(cap, 4);
            mov.annotation = annotation!(cap, 5);
            return Ok(mov);
        }

        Err(format!("could not parse: {}", value))
    }
}

#[cfg(test)]
mod tests;
