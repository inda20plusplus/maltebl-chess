pub mod chess_logic {
    struct ChessBoard {
        board: [[Option<Piece>; 8]; 8], //Hardcoded size, use hashmap?
        match_running: bool,
    }

    fn init_board() {}

    enum Color {
        White,
        Black,
    }

    enum PieceType {
        Pawn,
        Rook,
        Knight,
        Bishop,
        King,
        Queen,
    }

    struct Piece {
        color: Color,
        piece_type: PieceType,
        movement: ((i32, i32), Option<(i32, i32)>),
        has_moved: bool,
        moves_continous: bool,
        is_pawn: bool,
    }

    fn make_piece(p_type: PieceType, c: Color) -> Piece {
        match &p_type {
            PieceType::Pawn => Piece {
                color: c,
                piece_type: p_type,
                movement: ((0, 1), Some((1, 1))),
                has_moved: false,
                moves_continous: false,
                is_pawn: true,
            },
            PieceType::Rook => Piece {
                color: c,
                piece_type: p_type,
                movement: ((0, 1), None),
                has_moved: false,
                moves_continous: true,
                is_pawn: false,
            },
            PieceType::Knight => Piece {
                color: c,
                piece_type: p_type,
                movement: ((2, 1), Some((2, -1))),
                has_moved: false,
                moves_continous: false,
                is_pawn: false,
            },
            PieceType::Bishop => Piece {
                color: c,
                piece_type: p_type,
                movement: ((1, 1), None),
                has_moved: false,
                moves_continous: true,
                is_pawn: false,
            },
            PieceType::King => Piece {
                color: c,
                piece_type: p_type,
                movement: ((1, 1), Some((1, 0))),
                has_moved: false,
                moves_continous: false,
                is_pawn: false,
            },
            PieceType::Queen => Piece {
                color: c,
                piece_type: p_type,
                movement: ((1, 1), Some((1, 0))),
                has_moved: false,
                moves_continous: true,
                is_pawn: false,
            },
        }
    }

    pub fn translate_position(c: char, n: u32) -> Result<(u32, u32), String> {
        let pos_x = c as u32;
        if pos_x <= 96 || pos_x > 104 {
            return Err(String::from("tried to access non-existent boardspace"));
        }
        Ok((pos_x, n))
    }
}
