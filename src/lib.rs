pub mod console_display;
pub mod piece_logic;

pub mod chess_logic {
    use super::piece_logic::*;
    pub struct ChessBoard {
        board: [[Option<Piece>; 8]; 8], //Hardcoded size bad? //flatten to 64, get helper function turn (x,y)-> pos
        black_king: (usize, usize),
        white_king: (usize, usize),
        passant_connection: Option<((usize, usize), (usize, usize))>,
    }

    impl ChessBoard {
        pub fn add_piece(&mut self, piece: Piece, position: (usize, usize)) {
            if self.board[position.1][position.0].is_none() {
                self.board[position.1][position.0] = Some(piece);
            } else {
                panic!("Tried to add piece at non-empty space at {:?}", position)
            }
        }

        pub fn legal_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
            let mut results: Vec<(usize, usize)> = Vec::new();
            if let Some(piece) = self.ref_piece(position) {
                let mut legal_spaces: Vec<((usize, usize), Option<&Piece>)> = Vec::new();
                match piece.piece_type {
                    PieceType::Pawn => {
                        let color_modifier = if piece.color == Color::White { 1 } else { -1 };
                        if let Ok((pos, space)) = self.check_move(position, (0, color_modifier)) {
                            if space.is_none() {
                                results.push(pos);
                            }
                        }
                        if let Ok((pos, space)) = self.check_move(position, (1, color_modifier)) {
                            if space.is_some() && space.unwrap().color != piece.color {
                                results.push(pos);
                            }
                        }
                        if let Ok((pos, space)) = self.check_move(position, (-1, color_modifier)) {
                            if space.is_some() && space.unwrap().color != piece.color {
                                results.push(pos);
                            }
                        }
                    }
                    _ => {
                        let (movement1, movement2) = piece.movement;
                        legal_spaces.extend(self.check_around(
                            position,
                            movement1,
                            piece.moves_continous,
                        ));
                        if let Some(movement) = movement2 {
                            legal_spaces.extend(self.check_around(
                                position,
                                movement,
                                piece.moves_continous,
                            ));
                        }
                        for (position, space) in legal_spaces {
                            if let Some(p) = space {
                                if p.color == piece.color {
                                    continue;
                                }
                            }
                            results.push(position);
                        }
                    }
                }
            }
            results.sort();
            results.dedup();
            results
        }

        fn special_moves(&self, position: (usize, usize)) -> Vec<((usize, usize), (usize, usize))> {
            let mut special_moves: Vec<((usize, usize), (usize, usize))> = Vec::new();
            if let Some(piece) = self.ref_piece(position) {
                let (pos_x, pos_y) = position;
                if !piece.has_moved {
                    if piece.piece_type == PieceType::Pawn {
                        let color_modifier = if piece.color == Color::White { 2 } else { -2 };
                        if let Ok(target_space) = self.check_move(position, (0, color_modifier)) {
                            if target_space.1.is_none() {
                                special_moves.push((target_space.0, (pos_x, pos_y + 1)));
                            }
                        }
                    }
                    if piece.piece_type == PieceType::King {
                        let color_modifier = if piece.color == Color::White { 0 } else { 7 };
                        if let Some(space) = self.check_continous(position, (1, 0)).pop() {
                            if let Some(rook) = space.1 {
                                if rook.color == piece.color && !rook.has_moved {
                                    special_moves.push(((6, color_modifier), (5, color_modifier)));
                                }
                            }
                        }
                        if let Some(space) = self.check_continous(position, (-1, 0)).pop() {
                            if let Some(rook) = space.1 {
                                if rook.color == piece.color && !rook.has_moved {
                                    special_moves.push(((2, color_modifier), (3, color_modifier)));
                                }
                            }
                        }
                    }
                }
            }
            special_moves
        }

        fn check_around(
            &self,
            position: (usize, usize),
            moveset: (isize, isize),
            moves_continous: bool,
        ) -> Vec<((usize, usize), Option<&Piece>)> {
            let mut legal_spaces: Vec<((usize, usize), Option<&Piece>)> = Vec::new();
            let (move_x, move_y) = moveset;
            let directions = [
                (move_x, move_y),
                (-move_x, move_y),
                (move_x, -move_y),
                (-move_x, -move_y),
            ];
            for direction in directions.iter() {
                if moves_continous {
                    legal_spaces.extend(self.check_continous(position, *direction));
                } else if let Ok(target_space) = self.check_move(position, *direction) {
                    legal_spaces.push(target_space);
                }
            }
            legal_spaces
        }

        fn check_continous(
            &self,
            position: (usize, usize),
            direction: (isize, isize),
        ) -> Vec<((usize, usize), Option<&Piece>)> {
            let mut legal_spaces: Vec<((usize, usize), Option<&Piece>)> = Vec::new();
            if let Ok(target_space) = self.check_move(position, direction) {
                legal_spaces.push(target_space);
                if target_space.1.is_none() {
                    legal_spaces.extend(self.check_continous(target_space.0, direction));
                }
            }
            legal_spaces
        }

        fn check_move(
            &self,
            position: (usize, usize),
            moves: (isize, isize),
        ) -> Result<((usize, usize), Option<&Piece>), String> {
            let new_x = position.0 as isize + moves.0;
            let new_y = position.1 as isize + moves.1;
            if new_x < 0 || new_x >= 8 || new_y < 0 || new_y >= 8 {
                return Err("not valid movement".to_string());
            }
            let new_pos = (new_x as usize, new_y as usize);
            let target_space = self.ref_piece(new_pos);
            if target_space.is_some() {
                Ok((new_pos, target_space))
            } else {
                Ok((new_pos, None))
            }
        }

        pub fn move_piece(&mut self, move_from: (usize, usize), move_to: (usize, usize)) {
            let legal_moves = self.legal_moves(move_from);
            if legal_moves.contains(&move_to) {
                let piece: Piece = self.board[move_from.1][move_from.0].take().unwrap();
                self.board[move_to.1][move_to.0] = Some(piece);
            } else {
                println!(
                    "tried to do illegal move! Cannot move from {:?} to {:?}",
                    move_from, move_to
                );
            }
        }

        pub fn move_illegal(&mut self, piece_pos: (usize, usize), new_pos: (usize, usize)) {
            let piece: Piece = self.board[piece_pos.1][piece_pos.0].take().unwrap();
            self.board[new_pos.1][new_pos.0] = Some(piece);
        }

        fn is_threatened(&self, pos: (usize, usize), color: Color) -> bool {
            let mut direction = (0, 1);
            if self.is_threatened_from(pos, color, direction, true) {
                return true;
            }
            direction = (1, 1);
            if self.is_threatened_from(pos, color, direction, true) {
                return true;
            }
            direction = (1, 2);
            if self.is_threatened_from(pos, color, direction, false) {
                return true;
            }
            direction = (2, 1);
            if self.is_threatened_from(pos, color, direction, false) {
                return true;
            }
            false
        }

        fn is_threatened_from(
            &self,
            pos: (usize, usize),
            color: Color,
            direction: (isize, isize),
            check_continous: bool,
        ) -> bool {
            for space in self.check_around(pos, direction, check_continous) {
                if let Some(piece) = space.1 {
                    if piece.color != color && self.legal_moves(space.0).contains(&pos) {
                        return true;
                    }
                }
            }
            false
        }

        fn is_check_mate(&self) {}

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
            white_king: (0, 0),
            black_king: (0, 0),
            passant_connection: None,
        };
        place_pieces(&mut board, Color::White);
        place_pieces(&mut board, Color::Black);
        println!("printed board:");
        board
    }

    fn place_pieces(board: &mut ChessBoard, c: Color) {
        let mut y = if c == Color::White { 1 } else { 6 };
        for x in 0..8 {
            board.add_piece(piece_make(c, PieceType::Pawn), (x, y));
        }
        y = if c == Color::White { 0 } else { 7 };
        board.add_piece(piece_make(c, PieceType::Rook), (0, y));
        board.add_piece(piece_make(c, PieceType::Knight), (1, y));
        board.add_piece(piece_make(c, PieceType::Bishop), (2, y));
        board.add_piece(piece_make(c, PieceType::Queen), (3, y));
        board.add_piece(piece_make(c, PieceType::King), (4, y));
        if c == Color::White {
            board.white_king = (4, y)
        } else {
            board.black_king = (4, y)
        };
        board.add_piece(piece_make(c, PieceType::Bishop), (5, y));
        board.add_piece(piece_make(c, PieceType::Knight), (6, y));
        board.add_piece(piece_make(c, PieceType::Rook), (7, y));
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
