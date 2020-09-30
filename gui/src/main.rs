mod color_util;
mod delegate;
mod state;
mod tile;
mod ui;

use crate::delegate::Delegate;
use crate::state::AppState;
use crate::ui::main_ui;

use maltebl_chess::chess_game::*;

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
