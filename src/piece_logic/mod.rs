#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub movement: ((isize, isize), Option<(isize, isize)>),
    pub has_moved: bool,
    pub moves_continous: bool,
}

pub fn piece_make(color: Color, piece_type: PieceType) -> Piece {
    Piece {
        color,
        has_moved: false,
        movement: match &piece_type {
            PieceType::Pawn => ((0, 1), Some((1, 1))),
            PieceType::Rook => ((0, 1), None),
            PieceType::Knight => ((1, 2), Some((2, 1))),
            PieceType::Bishop => ((1, 1), None),
            PieceType::King => ((0, 1), Some((1, 1))),
            PieceType::Queen => ((0, 1), Some((1, 1))),
        },
        moves_continous: match &piece_type {
            PieceType::Pawn => false,
            PieceType::Rook => true,
            PieceType::Knight => false,
            PieceType::Bishop => true,
            PieceType::King => false,
            PieceType::Queen => true,
        },
        piece_type,
    }
}
