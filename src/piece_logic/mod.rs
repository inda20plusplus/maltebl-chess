use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}
#[derive(PartialEq, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub movement: ((isize, isize), Option<(isize, isize)>),
    pub has_moved: bool,
    pub moves_continous: bool,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol: char = if self.color == Color::White {
            match self.piece_type {
                PieceType::Pawn => '\u{265F}',
                PieceType::Rook => '\u{265C}',
                PieceType::Knight => '\u{265E}',
                PieceType::Bishop => '\u{265D}',
                PieceType::King => '\u{265A}',
                PieceType::Queen => '\u{265B}',
            }
        } else {
            match self.piece_type {
                PieceType::Pawn => '\u{2659}',
                PieceType::Rook => '\u{2656}',
                PieceType::Knight => '\u{2658}',
                PieceType::Bishop => '\u{2657}',
                PieceType::King => '\u{2654}',
                PieceType::Queen => '\u{2655}',
            }
        };
        write!(f, "{}", symbol)
    }
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
