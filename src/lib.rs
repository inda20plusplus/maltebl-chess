pub mod console_display;
pub mod piece_logic;

pub mod chess_logic {
    use super::piece_logic::*;
    pub struct ChessBoard {
        board: [[Option<Piece>; 8]; 8], //Hardcoded size bad?
        match_running: bool,
    }

    impl ChessBoard {
        pub fn add_piece(&mut self, piece: Piece) {
            let position = piece.position;
            if self.board[position.1][position.0].is_none() {
                self.board[position.1][position.0] = Some(piece);
            } else {
                panic!("Tried to add piece at non-empty space at {:?}", position)
            }
        }

        pub fn legal_moves(&self, piece: &Piece) -> Vec<(usize, usize)> {
            let mut legal_spaces: Vec<(usize, usize)> = Vec::new();
            if let PieceType::Pawn = piece.piece_type {
                self.check_move(
                    piece.color,
                    piece.position,
                    (0, 1),
                    false,
                    &mut legal_spaces,
                );
                if !piece.has_moved {
                    self.check_move(
                        piece.color,
                        piece.position,
                        (0, 2),
                        false,
                        &mut legal_spaces,
                    );
                }
                legal_spaces
            } else {
                self.check_around(
                    piece,
                    piece.movement.0,
                    piece.moves_continous,
                    &mut legal_spaces,
                );
                if let Some((moveset)) = piece.movement.1 {
                    self.check_around(piece, moveset, piece.moves_continous, &mut legal_spaces);
                }
                legal_spaces
            }
        }

        fn check_around(
            &self,
            piece: &Piece,
            moveset: (isize, isize),
            moves_continous: bool,
            legal_spaces: &mut Vec<(usize, usize)>,
        ) {
            let (move_x, move_y) = moveset;
            self.check_move(
                piece.color,
                piece.position,
                (move_x, move_y),
                piece.moves_continous,
                legal_spaces,
            );
            self.check_move(
                piece.color,
                piece.position,
                (-move_x, move_y),
                piece.moves_continous,
                legal_spaces,
            );
            self.check_move(
                piece.color,
                piece.position,
                (move_x, -move_y),
                piece.moves_continous,
                legal_spaces,
            );
            self.check_move(
                piece.color,
                piece.position,
                (-move_x, -move_y),
                piece.moves_continous,
                legal_spaces,
            );
            legal_spaces.sort();
            legal_spaces.dedup()
        }

        fn check_move(
            &self,
            piece_color: Color,
            position: (usize, usize),
            moves: (isize, isize),
            moves_continous: bool,
            legal_spaces: &mut Vec<(usize, usize)>,
        ) {
            let new_x = position.0 as isize + moves.0;
            let new_y = position.1 as isize + moves.1;
            if new_x < 0 || new_x >= 8 || new_y < 0 || new_y >= 8 {
                return;
            }
            let new_pos = (new_x as usize, new_y as usize);
            let target_space = self.ref_piece(new_pos);
            if target_space.is_some() {
                if target_space.unwrap().color == piece_color {
                    return;
                }
            } else if moves_continous {
                self.check_move(piece_color, new_pos, moves, moves_continous, legal_spaces)
            }
            legal_spaces.push(new_pos);
        }

        pub fn move_piece(&mut self, piece_pos: (usize, usize), new_pos: (usize, usize)) {
            let piece: Piece = self.board[piece_pos.1][piece_pos.0].take().unwrap();
            self.board[new_pos.1][new_pos.0] = Some(piece);
        }

        pub fn ref_board(&self) -> &[[Option<Piece>; 8]; 8] {
            &self.board
        }

        pub fn ref_piece(&self, position: (usize, usize)) -> Option<&Piece> {
            self.board[position.1][position.0].as_ref()
        }
    }

    pub fn init_board() -> ChessBoard {
        let mut board = ChessBoard {
            board: Default::default(),
            match_running: false,
        };
        place_pieces(&mut board, Color::White);
        place_pieces(&mut board, Color::Black);
        println!("printed board:");
        board
    }

    fn place_pieces(board: &mut ChessBoard, c: Color) {
        let mut y = if c == Color::White { 1 } else { 6 };
        for x in 0..8 {
            board.add_piece(piece_make(c, PieceType::Pawn, (x, y)));
        }
        y = if c == Color::White { 0 } else { 7 };
        board.add_piece(piece_make(c, PieceType::Rook, (0, y)));
        board.add_piece(piece_make(c, PieceType::Knight, (1, y)));
        board.add_piece(piece_make(c, PieceType::Bishop, (2, y)));
        board.add_piece(piece_make(c, PieceType::Queen, (3, y)));
        board.add_piece(piece_make(c, PieceType::King, (4, y)));
        board.add_piece(piece_make(c, PieceType::Bishop, (5, y)));
        board.add_piece(piece_make(c, PieceType::Knight, (6, y)));
        board.add_piece(piece_make(c, PieceType::Rook, (7, y)));
    }

    pub fn to_coords(c: char, n: usize) -> Result<(usize, usize), String> {
        let pos_x = c as usize - 96;
        if pos_x < 1 || pos_x > 8 || n < 1 || n > 8 {
            return Err(String::from("tried to access non-existent boardspace"));
        }
        Ok((pos_x, n))
    }

    pub fn to_notation(position: (usize, usize)) -> Result<(char, usize), String> {
        let (x, y) = position;
        if x < 1 || x > 8 || y < 1 || y > 8 {
            return Err(String::from("tried to access non-existent boardspace"));
        }
        Ok(((x + 96) as u8 as char, y))
    }
}
