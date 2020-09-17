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
            if let Some(piece) = space {
                print!("{}", piece);
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
