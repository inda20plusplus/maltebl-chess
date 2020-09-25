mod color_util;
mod delegate;
mod state;
mod style;
mod tile;
mod ui;

use crate::delegate::Delegate;
use crate::state::AppState;
use crate::ui::main_ui;

use maltebl_chess::chess_game::*;
use maltebl_chess::*;

use druid::{AppLauncher, Env, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let game = init_standard_chess();
    let data = AppState::new(game.get_board());
    let delegate = Delegate { game };

    let window = WindowDesc::new(main_ui)
        .title(|data: &AppState, _env: &Env| {
            format!(
                "Chess {}",
                match &data.message {
                    Some(t) => t.to_owned(),
                    None => "".to_owned(),
                }
            )
        })
        .resizable(false)
        .window_size((400.0, 400.0));

    let app = AppLauncher::with_window(window);
    app.delegate(delegate).launch(data)?;

    Ok(())
}

#[allow(unused)]
fn playground() -> Result<String, String> {
    let mut game = init_standard_chess();
    let cpos = (0 as usize, 1 as usize);
    let tpos = (0 as usize, 2 as usize);
    let command = format!("{} {}", to_notation(cpos)?, to_notation(tpos)?);
    let msg = game.move_piece(command.clone())?;
    // game.print_board();
    // let piece_char = game.get_piece((0, 0)).as_ref().map(|x| format!("{}", x)).unwrap_or(" ".to_owned());
    // println!("{}, {}, {}", piece_char, msg, command);
    Ok("".to_owned())
}
