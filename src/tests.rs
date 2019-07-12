
use super::*;

macro_rules! assert_all {
    ($mv:expr, $src:expr, $dst:expr, $piece:expr, $promo:expr, $ann:expr, $check:expr, $capt:expr) => {
        match $mv.move_kind {
            MoveKind::Normal(src, dst) => {
                assert_eq!(src, $src);
                assert_eq!(dst, $dst);
            },
            _ => assert!(false)
        }
        assert_eq!($mv.piece, $piece);
        assert_eq!($mv.promotion, $promo);
        assert_eq!($mv.annotation, $ann);
        assert_eq!($mv.check_type, $check);
        assert_eq!($mv.is_capture, $capt);
    };
}

macro_rules! assert_move {
    ($mv:expr, $src:expr, $dst:expr, $piece:expr) => {
        assert_all!($mv, $src, $dst, $piece, None, None, None, false);
    };
}
macro_rules! assert_capture {
    ($mv:expr, $src:expr, $dst:expr, $piece:expr) => {
        assert_all!($mv, $src, $dst, $piece, None, None, None, true);
    };
}

#[test]
fn test_castle_short() {
    let m = Move::parse("O-O").unwrap();
    assert_eq!(m.move_kind, MoveKind::Castle(CastleType::Kingside));
    assert_eq!(m.piece, Piece::King);
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.check_type, None);
    assert_eq!(m.is_capture, false);
}
#[test]
fn test_castle_long() {
    let m = Move::parse("O-O-O").unwrap();
    assert_eq!(m.move_kind, MoveKind::Castle(CastleType::Queenside));
    assert_eq!(m.piece, Piece::King);
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.check_type, None);
    assert_eq!(m.is_capture, false);
}
#[test]
fn test_pawn() {
    assert_move!(Move::parse("e4").unwrap(), 
        super::POS_NONE, 
        Position::of(4, 4), 
        Piece::Pawn);
}
#[test]
fn test_pawn_long() {
    assert_move!(Move::parse("e2e4").unwrap(), 
        Position::of(4, 6), 
        Position::of(4, 4), 
        Piece::Pawn);
}
#[test]
fn test_piece() {
    assert_move!(Move::parse("Qe4").unwrap(), 
        super::POS_NONE, 
        Position::of(4, 4), 
        Piece::Queen);
}
#[test]
fn test_piece_file() {
    assert_move!(Move::parse("Qbe4").unwrap(), 
        Position::new(Some(1), None), 
        Position::of(4, 4), 
        Piece::Queen);
}
#[test]
fn test_piece_rank() {
    assert_move!(Move::parse("Q1e4").unwrap(), 
        Position::new(None, Some(7)), 
        Position::of(4, 4), 
        Piece::Queen);
}
#[test]
fn test_piece_long() {
    assert_move!(Move::parse("Qb1e4").unwrap(), 
        Position::of(1, 7), 
        Position::of(4, 4), 
        Piece::Queen);
}
#[test]
fn test_pawn_capture() {
    assert_capture!(Move::parse("exd4").unwrap(), 
        Position::new(Some(4), None), 
        Position::of(3, 4), 
        Piece::Pawn);
}
#[test]
fn test_pawn_capture_promotion() {
    assert_all!(Move::parse("exd8=Q").unwrap(), 
        Position::new(Some(4), None), 
        Position::of(3, 0), 
        Piece::Pawn, 
        Some(Piece::Queen), 
        None, 
        None, 
        true);
}
#[test]
fn test_pawn_capture_long() {
    assert_capture!(Move::parse("e3xd4").unwrap(),
        Position::of(4, 5),
        Position::of(3, 4),
        Piece::Pawn);
}
#[test]
fn test_piece_capture() {
    assert_capture!(Move::parse("Rxh3").unwrap(),
        super::POS_NONE,
        Position::of(7, 5),
        Piece::Rook);
}
#[test]
fn test_piece_capture_file() {
    assert_capture!(Move::parse("Rexh3").unwrap(),
        Position::new(Some(4), None),
        Position::of(7, 5),
        Piece::Rook);
}
#[test]
fn test_piece_capture_rank() {
    assert_capture!(Move::parse("R1xh3").unwrap(),
        Position::new(None, Some(7)),
        Position::of(7, 5),
        Piece::Rook);
}
#[test]
fn test_piece_capture_long() {
    assert_capture!(Move::parse("Re3xh3").unwrap(),
        Position::of(4, 5),
        Position::of(7, 5),
        Piece::Rook);
}
#[test]
fn test_pawn_promotion() {
    assert_all!(Move::parse("d8=Q").unwrap(), 
        super::POS_NONE, 
        Position::of(3, 0), 
        Piece::Pawn, 
        Some(Piece::Queen), 
        None, 
        None, 
        false);
}

#[test]
fn test_compile() {
    let s = (Move {
        move_kind: MoveKind::Normal(Position::new(Some(4), None), Position::of(3,0)),
        piece: Piece::Pawn,
        promotion: Some(Piece::Queen),
        annotation: Some(Annotation::Interesting),
        check_type: Some(CheckType::Check),
        is_capture: true
    }).compile();

    assert_eq!(s, "exd8=Q+?!");
}