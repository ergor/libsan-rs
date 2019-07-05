
pub trait StrEnum {
    fn to_str(&self) -> &str;
    fn from_str<T>(value: &str) -> Result<T, &str>;
}

pub enum Piece {
    Pawn,
    Bishop,
    King,
    Knight,
    Queen,
    Rook
}

impl StrEnum for Piece {
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
}

pub enum Annotation {
    Blunder,
    Mistake,
    Interesting,
    Good,
    Brilliant
}

impl StrEnum for Annotation {
    fn to_str(&self) -> &str {
        match self {
            Annotation::Blunder => "??",
            Annotation::Mistake => "?",
            Annotation::Interesting => "?!",
            Annotation::Good => "!",
            Annotation::Brilliant => "!!"
        }
    }
}

pub enum CastleType {
    Kingside,
    Queenside
}

impl StrEnum for CastleType {
    fn to_str(&self) -> &str {
        match self {
            CastleType::Kingside => "O-O",
            CastleType::Queenside => "O-O-O"
        }
    }

    fn from_str<CastleType>(value: &str) -> Result<CastleType, &str> {
        match value {
            "O-O" => Ok(CastleType::Kingside),
            "O-O-O" => Ok(Queenside),
            _ => Err("not a castling move")
        }
    }
}

pub struct Position {
    x: usize,
    y: usize
}

pub enum MoveType {
    Normal(Option<Position>, Option<Position>), // src_pos, dst_pos
    Castle(CastleType),
    Undefined
}

pub struct Move {
    pub move_type: MoveType,
    pub piece: Option<Piece>,
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
            annotation: None,
            is_capture: false,
            is_check: false,
            is_check_mate: false
        }
    }
}
