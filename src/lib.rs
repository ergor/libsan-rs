
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

macro_rules! promotion {
    ($cap:expr, $i:expr) => {
        Piece::from_str($cap.get($i).map_or("fail", |v| v.as_str())).ok();
    };
}

pub const POS_NONE: Position = Position {x: None, y: None};

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

#[cfg(test)]
mod tests;
