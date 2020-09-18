use super::piece_logic::*;

pub fn print_board(board: &[[Option<Piece>; 8]; 8]) {
    for y in (0..board.len()).rev() {
        for x in 0..board[0].len() {
            print!("|");
            if let Some(piece) = &board[y][x] {
                print!("{}", piece);
            } else {
                print!(" ")
            }
            print!("|");
        }
        println!();
        println!("------------------------");
    }
}
