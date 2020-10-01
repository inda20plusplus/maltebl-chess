use std::sync::Arc;

use maltebl_chess::board_logic::Board;
use maltebl_chess::piece_logic::Piece;

use druid::Data;

#[derive(Data, Copy, Clone, Debug, PartialEq)]
pub struct Position(pub i32, pub i32);

#[derive(Data, Clone)]
pub struct AppState {
    pub board: Arc<Board>,
    pub origin: Option<Position>,
    pub message: Option<String>,
}

impl AppState {
    pub fn new(board: Board) -> Self {
        Self {
            board: Arc::new(board),
            origin: None,
            message: None,
        }
    }
    pub fn get_piece(&self, position: Position) -> &Option<Piece> {
        let cpos = (position.0 as usize, position.1 as usize);
        &self.board[cpos.1][cpos.0]
    }
}
