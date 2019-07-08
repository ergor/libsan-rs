
pub trait StrEnum {
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

#[derive(Debug, Eq, PartialEq)]
pub enum MoveType {
    Normal(Position, Position), // src_pos, dst_pos
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
}
