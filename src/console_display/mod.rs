use super::piece_logic::*;

pub fn print_board(
    board: &[[Option<Piece>; 8]; 8], /*, marked_spots: Option<Vec<(usize, usize)>>*/
) {
    // let mut x = 0;
    //let mut y = 0;
    for row in board {
        for space in row {
            print!("|");
            // if has_marked_spots && marked_spots.unwrap().contains(&(x, y)) {
            //     print!("(");
            // } else {
            //     print!(" ")
            // }
            if space.is_some() {
                match space.as_ref().unwrap().piece_type {
                    PieceType::Pawn => print!("O"),
                    PieceType::Rook => print!("W"),
                    PieceType::Knight => print!("R"),
                    PieceType::Bishop => print!("I"),
                    PieceType::King => print!("K"),
                    PieceType::Queen => print!("Q"),
                }
            } else {
                print!(" ")
            }
            // if marked_spots.unwrap().contains(&(x, y)) {
            //     print!("(");
            // } else {
            //     print!(" ")
            // }
            print!("|");
            //   x += 1;
        }
        println!();
        println!("------------------------");
        // y += 1;
    }
}
