use std::sync::mpsc::Sender;
use std::sync::Arc;

use crate::state::AppState;

use maltebl_chess::{chess_game::ChessGame, piece_logic::Color};

use druid::*;

pub mod action {
    use druid::Selector;

    pub const MAKE_MOVE: Selector<String> = Selector::new("make_move");
    pub const MAKE_MOVE_FROM_NET: Selector<String> = Selector::new("make_move_from_net");
}

pub struct Delegate {
    pub game: ChessGame,
    pub single_player: Option<Color>,
    pub net_sender: Option<Sender<String>>,
}

impl Delegate {
    fn attempt_move_from_ui(&mut self, command: &str) -> Result<String, String> {
        if let Some(local_player) = self.single_player {
            if local_player != self.game.current_player() {
                return Err("not your (local player) turn".to_owned());
            }

            let sender = self
                .net_sender
                .as_ref()
                .ok_or_else(|| "no net_sender".to_owned())?;
            sender
                .send(command.to_owned())
                .map_err(|_| "unable to send".to_owned())?;
        }

        // TODO: + send move to network + only apply if ok
        // for now: just assume everything went smooth

        self.game.move_piece(command.to_owned())
    }
    // if err: send decline, if ok; pass along
    fn attempt_move_from_network(&mut self, command: &str) -> Result<String, String> {
        let local_player = self
            .single_player
            .ok_or_else(|| "single_player_white not set".to_owned())?;
        if local_player == self.game.current_player() {
            return Err("local players turn".to_owned());
        }

        // TODO: validate move + answer ok/decline + perform if ok
        // for now: just assume everything went smooth

        let sender = self
            .net_sender
            .as_ref()
            .ok_or_else(|| "no net_sender".to_owned())?;
        sender
            .send("ok".to_owned())
            .map_err(|_| "unable to send".to_owned())?;

        self.game.move_piece(command.to_owned())
    }
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
            data.message = Some(match self.attempt_move_from_ui(command) {
                Err(txt) => txt,
                Ok(txt) => txt,
            });
            data.board = Arc::new(self.game.get_board());
            return true;
        } else if let Some(command) = cmd.get(action::MAKE_MOVE_FROM_NET) {
            data.message = Some(match self.attempt_move_from_network(command) {
                Err(_txt) => "".to_owned(),
                Ok(txt) => txt,
            });
            data.board = Arc::new(self.game.get_board());
            return true;
        }
        true
    }
}
