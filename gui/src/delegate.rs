use crate::state::AppState;

use maltebl_chess::chess_game::ChessGame;

use druid::*;

use std::sync::Arc;

pub mod action {
    use druid::Selector;

    pub const MAKE_MOVE: Selector<String> = Selector::new("make_move");
}

pub struct Delegate {
    pub game: ChessGame,
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        if let Some(command) = cmd.get(action::MAKE_MOVE) {
            let msg = self.game.move_piece(command.to_owned());
            data.message = Some(match msg {
                Err(txt) => txt.to_owned(),
                _ => "".to_owned(),
            });
            data.board = Arc::new(self.game.get_board());
            return true;
        }
        true
    }
}
