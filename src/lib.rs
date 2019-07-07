
/// ported from https://github.com/chesszebra/standard-algebraic-notation

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_castle_short() {
        let m = super::parse("O-O");
        assert_eq!(m.move_type, MoveType::Castle(CastleType::Kingside));
        assert_eq!(m.piece, None);
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_castle_long() {
        let m = super::parse("O-O-O");
        assert_eq!(m.move_type, MoveType::Castle(CastleType::Queenside));
        assert_eq!(m.piece, None);
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_pawn() {
        let m = super::parse("e4");
        match m.move_type {
            MoveType::Normal(src, dst) => {
                assert_eq!(src, super::pos_none!());
                assert_eq!(dst, Some(Position {x:4, y:4}));
            },
            _ => assert!(false)
        }
        assert_eq!(m.piece, Some(Piece::Pawn));
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_pawn_long() {
        let m = super::parse("e2e4");
        match m.move_type {
            MoveType::Normal(src, dst) => {
                assert_eq!(src, Some(Position {x:4, y:6}));
                assert_eq!(dst, Some(Position {x:4, y:4}));
            },
            _ => assert!(false)
        }
        assert_eq!(m.piece, Some(Piece::Pawn));
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_piece() {
        let m = super::parse("Qe4");
        match m.move_type {
            MoveType::Normal(src, dst) => {
                assert_eq!(src, None);
                assert_eq!(dst, Some(Position {x:4, y:4}));
            },
            _ => assert!(false)
        }
        assert_eq!(m.piece, Some(Piece::Queen));
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
}

mod san;
use san::*;
use regex::Regex;

macro_rules! pos {
    ($cap:expr, $file:expr, $rank:expr) => {
        {
            Position {
                x: Some($cap[$file].chars().next().unwrap() as usize - 0x61),
                y: Some(7 - ($cap[$rank].parse::<usize>().unwrap() - 1))
            }
        }
    };
}

macro_rules! pos_none {
    () => {
        {
            Position {
                x: None,
                y: None
            }
        }
    };
}

macro_rules! file {
    ($cap:expr, $file:expr) => {
        {
            Position {
                x: Some($cap[$file].chars().next().unwrap() as usize - 0x61),
                y: None
            }
        }
    };
}

pub fn parse(value: &str) -> Move {
    let mut mov = Move::new();

    // Check for castling:
    let re = Regex::new(r"^(O-O|O-O-O)(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.move_type = MoveType::Castle(CastleType::from_str(&cap[1]).unwrap());
        mov.is_check = cap.get(2).map_or("", |v| v.as_str()) == "+";
        mov.is_check_mate = cap.get(2).map_or("", |v| v.as_str()) == "#";
        mov.annotation = Annotation::from_str(cap.get(3).map_or("", |v| v.as_str())).ok();
        return mov;
    }

    // Pawn movement:
    let re = Regex::new(r"^([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.piece = Some(Piece::Pawn);
        mov.move_type = MoveType::Normal(pos_none!(), pos!(cap, 1, 2));
        mov.is_check = cap.get(3).map_or("", |v| v.as_str()) == "+";
        mov.is_check_mate = cap.get(3).map_or("", |v| v.as_str()) == "#";
        mov.annotation = Annotation::from_str(cap.get(4).map_or("", |v| v.as_str())).ok();
        return mov;
    }

    // Pawn movement (long san):
    let re = Regex::new(r"^([a-h])([1-8])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.piece = Some(Piece::Pawn);
        mov.move_type = MoveType::Normal(pos!(cap, 1, 2), pos!(cap, 3, 4));
        mov.is_check = cap.get(5).map_or("", |v| v.as_str()) == "+";
        mov.is_check_mate = cap.get(5).map_or("", |v| v.as_str()) == "#";
        mov.annotation = Annotation::from_str(cap.get(6).map_or("", |v| v.as_str())).ok();
        return mov;
    }

    // Piece movement:
    let re = Regex::new(r"^([KQBNR])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.piece = Piece::from_str(&cap[1]).ok();
        mov.move_type = MoveType::Normal(pos_none!(), pos!(cap, 2, 3));
        mov.is_check = cap.get(4).map_or("", |v| v.as_str()) == "+";
        mov.is_check_mate = cap.get(4).map_or("", |v| v.as_str()) == "#";
        mov.annotation = Annotation::from_str(cap.get(5).map_or("", |v| v.as_str())).ok();
        return mov;
    }

    // Piece movement from a specific column:
    let re = Regex::new(r"^([KQBNR])([a-h])([a-h])([1-8])(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.piece = Piece::from_str(&cap[1]).ok();
        mov.move_type = MoveType::Normal(file!(cap, 2), pos!(cap, 3, 4));
        mov.is_check = cap.get(5).map_or("", |v| v.as_str()) == "+";
        mov.is_check_mate = cap.get(5).map_or("", |v| v.as_str()) == "#";
        mov.annotation = Annotation::from_str(cap.get(6).map_or("", |v| v.as_str())).ok();
        return mov;
    }

    panic!("could not parse: {}", value);
}
