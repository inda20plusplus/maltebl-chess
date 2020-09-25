use crate::color_util::ColorUtil;
use crate::delegate::action;
use crate::state::{AppState, Position};
use crate::style::*;
use crate::tile::Tile;

use maltebl_chess::{
    piece_logic::{self, Piece, PieceType},
    to_notation,
};

use druid::{widget::*, WidgetExt, *};

pub fn main_ui() -> impl Widget<AppState> {
    make_board()
}

fn make_board() -> impl Widget<AppState> {
    let make_row = |y: i32| {
        (0..NUM_COLS).fold(Flex::row(), |col, x| {
            col.with_flex_child(make_tile(Position(x, y)), 1.0)
        })
    };

    let make_rows = || {
        (0..NUM_ROWS).fold(Flex::column(), |row, y| {
            row.with_flex_child(make_row(y), 1.0)
        })
    };

    let board = make_rows()
        .center()
        .padding(15.0)
        .background(Color::from_rgba32_u32(0x332205FF));

    board
}

fn make_tile(pos: Position) -> impl Widget<AppState> {
    let label = Label::dynamic(move |data: &AppState, _| {
        match data.get_piece(pos) {
            None => "",
            Some(piece) => piece_char(piece),
        }
        .to_owned()
    })
    .with_text_size(27.);

    let tile = Tile::new(pos, label)
        .on_click(move |ctx, data, _env| {
            match data.origin {
                None => {
                    data.origin = Some(pos);
                    data.message = None;
                    // data.origin_available_moves = Some(Arc::new(data.game.chess_board.get_moves(cpos)));
                }
                Some(prev) => {
                    let is_target = prev != pos;
                    if is_target {
                        let cpos = (prev.0 as usize, prev.1 as usize);
                        let tpos = (pos.0 as usize, pos.1 as usize);

                        let mut doit = || -> Result<String, String> {
                            let command = format!("{} {}", to_notation(cpos)?, to_notation(tpos)?);
                            ctx.submit_command(Command::new(action::MAKE_MOVE, command), None);
                            Ok("".to_owned())
                        };

                        let txt = match doit() {
                            Err(inner) => format!("Error: {}", inner),
                            Ok(inner) => format!("{}", inner),
                        };

                        data.message = if txt.len() > 0 { Some(txt) } else { None };
                    };
                    data.origin = None;
                }
            }
        })
        .env_scope(move |env, data: &AppState| {
            let white = match data.get_piece(pos) {
                None => true,
                Some(piece) => match piece.color {
                    piece_logic::Color::White => true,
                    piece_logic::Color::Black => false,
                },
            };
            env.set(
                theme::LABEL_COLOR,
                if white {
                    ColorUtil::hsl(0.1, 0.15, 0.7)
                } else {
                    ColorUtil::hsl(0.1, 0.3, 0.3)
                },
            );
        });

    tile
}

fn piece_char(piece: &Piece) -> &str {
    match piece.piece_type {
        PieceType::King => "♚",
        PieceType::Queen => "♛",
        PieceType::Rook => "♜",
        PieceType::Knight => "♞",
        PieceType::Bishop => "♝",
        PieceType::Pawn => "♟︎",
    }
}
