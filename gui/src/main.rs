#![allow(unused)]

use maltebl_chess::chess_game::*;
use maltebl_chess::*;

fn main() {
    playground().unwrap();
}

fn playground() -> Result<String, String> {
    let mut game = init_standard_chess();
    let cpos = (0 as usize, 1 as usize);
    let tpos = (0 as usize, 2 as usize);
    let command = format!("{} {}", to_notation(cpos)?, to_notation(tpos)?);
    let msg = game.move_piece(command.clone())?;
    // game.print_board();
    let piece_char = game.get_piece((0, 0)).as_ref().map(|x| format!("{}", x)).unwrap_or(" ".to_owned());
    println!("{}, {}, {}", piece_char, msg, command);
    Ok("".to_owned())
}
