pub mod board_logic;
pub mod console_display;
pub mod piece_logic;

/// Engine for the boardgame "chess"
///
/// Minimal interaction example:
/// ```
/// fn example() -> Result<String, String> {
/// #    use maltebl_chess::{chess_game::*, *};
///     let mut game = init_standard_chess();
///
///     // construct command from clicked tiles
///     let origin = (0 as usize, 1 as usize);
///     let target = (0 as usize, 2 as usize);
///     let command = format!("{} {}", to_notation(origin)?, to_notation(target)?);
///
///     let msg = game.move_piece(command.clone())?;
///
///     game.print_board();
///     
///     // get the character for a piece at a specific position
///     let piece_char = game.get_board()[0][0].as_ref()
///         .map(|p| format!("{}", p))
///         .unwrap_or(" ".to_owned());
///
///     println!("{}, {}, {}", piece_char, msg, command);
///     Ok("".to_owned())
/// }
/// example().unwrap();
/// ````
pub mod chess_game {
    use super::*;
    use crate::{board_logic::*, piece_logic::*};

    pub struct ChessGame {
        chess_board: ChessBoard,
        history: Vec<String>,
        turn: (Color, usize),
    }

    impl ChessGame {
        pub fn get_board(&self) -> Board {
            self.chess_board.get_board()
        }

        pub fn pick_piece(&self, input: String) -> Result<Vec<String>, String> {
            let piece_position = to_coords(input).expect("Error:");
            if let Some(piece) = self.chess_board.ref_piece(piece_position) {
                if piece.color != self.turn.0 {
                    return Err("That's not your piece!".to_string());
                }
            } else {
                return Err(format!("There is no piece at {:?}", piece_position));
            }
            let mut possible_moves: Vec<String> = Vec::new();
            for (mov, _) in self.chess_board.get_moves(piece_position) {
                possible_moves.push(to_notation(mov).unwrap());
            }
            Ok(possible_moves)
        }

        pub fn current_player(&self) -> Color {
            self.turn.0
        }

        pub fn move_piece(&mut self, input: String) -> Result<String, String> {
            if input.len() == 5 {
                let mut input = input.split_whitespace();
                let move_from = to_coords(input.next().unwrap().to_string())?;
                let move_to = to_coords(input.next().unwrap().to_string())?;
                if let Some(piece) = self.chess_board.ref_piece(move_from) {
                    if piece.color != self.current_player() {
                        return Err("That is not your piece!".to_string());
                    }
                } else {
                    return Err(format!("There is no piece at {:?}", move_from));
                }
                let mut result = self.chess_board.move_piece(move_from, move_to);
                if result.is_ok() {
                    let mov = result.clone().unwrap();
                    let mov = mov.trim();
                    let mut history = format!("{}. ", self.turn.1);
                    history.push_str(mov);
                    self.history.push(history);
                    self.turn = (
                        match self.current_player() {
                            Color::Black => Color::White,
                            Color::White => Color::Black,
                        },
                        1 + self.turn.1,
                    );
                    if self.chess_board.is_checked(self.current_player()) {
                        result = Ok(result.unwrap() + " Check!");
                    }
                    if self.chess_board.is_checkmate(self.current_player()) {
                        return Ok("Game is over! It's a checkmate!".to_string());
                    }
                }
                result
            } else {
                Err("Error: enter move as e.g:a4 a3".to_string())
            }
        }

        pub fn promotion(&mut self, input: String) -> Result<String, String> {
            if input.len() == 3 {
                let mut chars = input.chars();
                let position = to_coords(format!(
                    "{}{}",
                    chars.next().unwrap(),
                    chars.next().unwrap()
                ))?;
                let piece_type = match chars.next() {
                    Some('Q') => PieceType::Queen,
                    Some('B') => PieceType::Bishop,
                    Some('N') => PieceType::Knight,
                    Some('R') => PieceType::Rook,
                    _ => PieceType::Pawn,
                };
                if piece_type == PieceType::Pawn {
                    return Err("Must provide promotion input as e.g:a8Q".to_string());
                }
                self.chess_board.promote(position, piece_type)
            } else {
                Err("Must provide promotion input as e.g:a8Q".to_string())
            }
        }
        pub fn print_board(&self) {
            console_display::print_board(self.chess_board.ref_board());
        }
    }
    pub fn init_standard_chess() -> ChessGame {
        let mut board = init_board();
        board.standard_pieces(Color::White);
        board.standard_pieces(Color::Black);
        ChessGame {
            chess_board: board,
            history: Vec::new(),
            turn: (Color::White, 1),
        }
    }
}

pub fn to_coords(input: String) -> Result<(usize, usize), String> {
    if input.len() == 2 {
        let mut input = input.chars();
        let mut pos_x = input.next().unwrap() as isize - 96;
        let mut pos_y: isize = input.next().unwrap().to_string().parse().unwrap();
        pos_x -= 1;
        pos_y -= 1;
        if pos_x < 0 || pos_x > 7 || pos_y < 0 || pos_y > 7 {
            println!("{}{}", pos_x, pos_y);
            return Err(String::from("tried to access non-existent boardspace"));
        }
        Ok((pos_x as usize, pos_y as usize))
    } else {
        Err(String::from(
            "invalid notation, cannot find coords on board",
        ))
    }
}

pub fn to_notation(position: (usize, usize)) -> Result<String, String> {
    let (x, y) = position;
    if x > 8 || y > 8 {
        return Err(String::from("tried to access non-existent boardspace"));
    }
    Ok(format!("{}{}", (x + 97) as u8 as char, y + 1))
}
