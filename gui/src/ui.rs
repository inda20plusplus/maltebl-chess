use crate::color_util::ColorUtil;
use crate::delegate::action;
use crate::state::{AppState, Position};
use crate::tile::Tile;

use maltebl_chess::{
    piece_logic::{self, Piece, PieceType},
    to_notation,
};

use druid::{widget::*, WidgetExt, *};

const NUM_ROWS: i32 = 8;
const NUM_COLS: i32 = 8;

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
    // fallback if no svg?
    // let label = Label::dynamic(move |data: &AppState, _| {
    //     match data.get_piece(pos) {
    //         None => "",
    //         Some(piece) => piece_char(piece),
    //     }
    //     .to_owned()
    // })
    // .with_text_size(27.);

    let icon: Tile2<AppState> = Tile2::new(
        pos,
        Box::new(|pos, data| {
            data.get_piece(pos)
                .as_ref()
                .and_then(|piece| Some((piece.piece_type.clone(), piece.color)))
        }),
    );

    let tile = Tile::new(pos, icon).on_click(move |ctx, data, _env| {
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
    });

    tile
}

type PieceVisual = Option<(PieceType, piece_logic::Color)>;
type PieceVisualMaker<T> = Box<dyn Fn(Position, &T) -> PieceVisual>;
struct Tile2<T: Data> {
    position: Position,
    inner: Option<Align<T>>,
    piece: PieceVisual,
    piece_maker: PieceVisualMaker<T>,
}
impl<T: Data> Tile2<T> {
    pub fn new(position: Position, piece_maker: PieceVisualMaker<T>) -> Self {
        Self {
            position,
            inner: None,
            piece: None,
            piece_maker,
        }
    }
    fn update_inner(&mut self, data: &T) -> bool {
        let new_type = (*self.piece_maker)(self.position, data);
        if new_type == self.piece {
            return false;
        }

        self.piece = new_type;
        self.inner = self.piece.as_ref().map(|t| {
            let color = match t.1 {
                piece_logic::Color::White => ColorUtil::hsl(0.1, 0.17, 0.72),
                piece_logic::Color::Black => ColorUtil::hsl(0.1, 0.3, 0.3),
            };

            let color = format!("{:?}", color);
            let color = &color[..color.len() - 2];
            let svg_data = piece_svg_colored(&t.0, color).parse::<SvgData>();
            Svg::new(svg_data.unwrap()).fix_width(30.0).center()
        });
        true
    }
}

impl<T: Data> Widget<T> for Tile2<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, _env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.update_inner(data);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, _env: &Env) {
        if self.update_inner(data) {
            ctx.request_layout();
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        if let Some(a) = &mut self.inner {
            a.layout(layout_ctx, bc, data, env)
        } else {
            bc.max()
        }
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Some(a) = &mut self.inner {
            a.paint(ctx, data, env)
        }
    }
}

#[allow(dead_code)]
fn piece_char(piece: &Piece) -> &str {
    match piece.piece_type {
        PieceType::King => "K",
        PieceType::Queen => "Q",
        PieceType::Rook => "R",
        PieceType::Knight => "N",
        PieceType::Bishop => "B",
        PieceType::Pawn => "P",
    }
}

fn piece_svg_raw(piece: &PieceType) -> &str {
    match piece {
        PieceType::King => include_str!("../assets/k.svg"),
        PieceType::Queen => include_str!("../assets/q.svg"),
        PieceType::Rook => include_str!("../assets/r.svg"),
        PieceType::Knight => include_str!("../assets/n.svg"),
        PieceType::Bishop => include_str!("../assets/b.svg"),
        PieceType::Pawn => include_str!("../assets/p.svg"),
    }
}

fn piece_svg_colored(piece: &PieceType, color: &str) -> String {
    let raw = piece_svg_raw(piece);
    let old_start_end = raw.find('>').expect("faulty svg?");

    let margin = 0;
    let org_size = (24, 24);
    let new_start = format!("<svg fill=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.1\" viewBox=\"{} {} {} {}\" xml:space=\"preserve\"", color, 0, 0, org_size.0+margin, org_size.1+margin);

    format!("{}{}", new_start, &raw[old_start_end..])
}
