
/// ported from https://github.com/chesszebra/standard-algebraic-notation

mod san;
use san::*;
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

const POS_NONE: Position = Position {x: None, y: None};

pub fn parse(value: &str) -> Move {
    let mut mov = Move::new();

    // Check for castling:
    let re = Regex::new(r"^(O-O|O-O-O)(\+|\#)?(\?\?|\?|\?!|!|!!)?$").unwrap();
    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        mov.move_type = MoveType::Castle(CastleType::from_str(&cap[1]).unwrap());
        mov.is_check = is_check!(cap, 2);
        mov.is_check_mate = is_check_mate!(cap, 2);
        mov.annotation = annotation!(cap, 3);
        return mov;
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
        return mov;
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
        return mov;
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
        return mov;
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
        return mov;
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
        return mov;
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
        return mov;
    }

    panic!("could not parse: {}", value);
}

/* TESTS ---------------------------------------------------------------------*/
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
                assert_eq!(src, super::POS_NONE);
                assert_eq!(dst, Position::of(4, 4));
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
                assert_eq!(src, Position::of(4, 6));
                assert_eq!(dst, Position::of(4, 4));
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
                assert_eq!(src, super::POS_NONE);
                assert_eq!(dst, Position::of(4, 4));
            },
            _ => assert!(false)
        }
        assert_eq!(m.piece, Some(Piece::Queen));
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_piece_file() {
        let m = super::parse("Qbe4");
        match m.move_type {
            MoveType::Normal(src, dst) => {
                assert_eq!(src, Position::new(Some(1), None));
                assert_eq!(dst, Position::of(4, 4));
            },
            _ => assert!(false)
        }
        assert_eq!(m.piece, Some(Piece::Queen));
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_piece_rank() {
        let m = super::parse("Q1e4");
        match m.move_type {
            MoveType::Normal(src, dst) => {
                assert_eq!(src, Position::new(None, Some(7)));
                assert_eq!(dst, Position::of(4, 4));
            },
            _ => assert!(false)
        }
        assert_eq!(m.piece, Some(Piece::Queen));
        assert_eq!(m.annotation, None);
        assert_eq!(m.is_capture, false);
        assert_eq!(m.is_check, false);
        assert_eq!(m.is_check_mate, false);
    }
    #[test]
    fn test_piece_long() {
        let m = super::parse("Qb1e4");
        match m.move_type {
            MoveType::Normal(src, dst) => {
                assert_eq!(src, Position::new(Some(1), Some(7)));
                assert_eq!(dst, Position::of(4, 4));
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
