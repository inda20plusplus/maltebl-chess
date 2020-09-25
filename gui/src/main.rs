#![allow(unused)]

use maltebl_chess::chess_game::*;
use maltebl_chess::*;
use maltebl_chess::piece_logic::{self, Piece, PieceType};
use maltebl_chess::board_logic::Board;

use druid::widget::*;
use druid::WidgetExt;
use druid::*;

use druid::piet::Color;

use std::rc::Rc;
use std::sync::Arc;

const NUM_ROWS: i32 = 8;
const NUM_COLS: i32 = 8;

fn main() -> Result<(), PlatformError> {
    let game = init_standard_chess();

    let data = AppState::new(game.get_board());
    let window = WindowDesc::new(main_ui)
        .title(|data: &AppState, _env: &Env| format!("Chess {}", match &data.message {
            Some(t)=> t.to_owned(),
            None=> "".to_owned(),
        }))
        .resizable(false)
        .window_size((300.0, 300.0));

    // let tile = &chess.chess_board.board[0][0];
    // println!("{}", tile.clone().unwrap());
    let app = AppLauncher::with_window(window);
    let delegate = Delegate { game };
    app.delegate(delegate).launch(data);
    Ok(())
}

pub const MAKE_MOVE: Selector<String> = Selector::new("make_move");

struct Delegate {
    game: ChessGame,
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
        if let Some(command) = cmd.get(MAKE_MOVE) {
            let msg = self.game.move_piece(command.to_owned());
            println!("{}: {:?}", command, msg);
            data.message = Some(match msg {
                Err(txt)=> txt.to_owned(),
                _=> "".to_owned(),
            });
            data.board = Arc::new(self.game.get_board());
            return true;
        }
        true
    }
}

fn wrapped_slow_function(sink: ExtEventSink, number: u32) {
    // thread::spawn(move || {
    //     let number = slow_function(number);
    //     sink.submit_command(FINISH_SLOW_FUNCTION, number, Target::Auto)
    //         .expect("command failed to submit");
    // });
}

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

#[derive(Data, Copy, Clone, Debug, PartialEq)]
struct Position(i32, i32);
#[derive(Data, Clone)]
struct AppState {
    board: Arc<Board>,
    origin: Option<Position>,
    message: Option<String>,
}

impl AppState {
    fn new(board: Board) -> Self {
        Self {
            board: Arc::new(board),
            origin: None,
            message: None,
        }
    }
    fn get_piece(&self, position: Position) -> &Option<Piece> {
        let cpos = (position.0 as usize, position.1 as usize);
        &self.board[cpos.1][cpos.0]
    }
}

fn main_ui() -> impl Widget<AppState> {
    let make_tile = |pos: Position| {
        // let _b = Button::dynamic(move |data: &AppState, _| {
        //     let cpos = (pos.0 as usize, pos.1 as usize);
        //     data.game
        //         .get_piece(cpos)
        //         .as_ref()
        //         .map(|x| format!("{}", x))
        //         .unwrap_or(" ".to_owned())
        // })
        // // .on_click()
        // .env_scope(move |env, data: &AppState| {
        //     let is_selected = data.origin.filter(|a| *a == pos).is_some();
        //     let checkerboard = pos.0 % 2 == pos.1 % 2;
        //     let col = if checkerboard { 0x000000FF } else { 0x777777FF };
        //     let col = col + if is_selected { 0x33333300 } else { 0 };
        //     let col = Color::from_rgba32_u32(col);

        //     env.set(theme::BUTTON_DARK, col.clone());
        //     env.set(theme::BUTTON_LIGHT, col);
        //     env.set(theme::BUTTON_BORDER_WIDTH, 0.0);
        //     env.set(theme::BUTTON_BORDER_RADIUS, 0.0);
        // });


        let label = Label::dynamic(move |data: &AppState, _| {
            match data.get_piece(pos) {
                None=> "",
                Some(piece)=> {
                    let text = match piece.piece_type {
                        PieceType::King => "♚",
                        PieceType::Queen => "♛",
                        PieceType::Rook => "♜",
                        PieceType::Knight => "♞",
                        PieceType::Bishop => "♝",
                        PieceType::Pawn => "♟︎",
                    };
                    text
                },
            }.to_owned()
        });
        let tile = Tile::new(pos, label)
        .on_click(move |ctx, data, env| {
            match data.origin {
                None => {
                    let cpos = (pos.0 as usize, pos.1 as usize);
                    data.origin = Some(pos);
                    // data.origin_available_moves = Some(Arc::new(data.game.chess_board.get_moves(cpos)));
                }
                Some(prev) => {
                    let is_target = prev != pos;
                    if is_target {
                        // TODO: emit action: target: pos
                        let cpos = (prev.0 as usize, prev.1 as usize);
                        let tpos = (pos.0 as usize, pos.1 as usize);

                        let mut doit = || -> Result<String, String> {
                            let command = format!("{} {}", to_notation(cpos)?, to_notation(tpos)?);
                            ctx.submit_command(Command::new(MAKE_MOVE, command), None);
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
        }).env_scope(move |env, data: &AppState| {
            let white = match data.get_piece(pos) {
                None=> true,
                Some(piece)=> {
                    match piece.color {
                        piece_logic::Color::White=> true,
                        piece_logic::Color::Black=> false,
                    }      
                },
            };
            env.set(theme::LABEL_COLOR, if white {
                ColorUtil::hsl(0.1, 0.05, 0.8)
            } else {
                ColorUtil::hsl(0.1, 0.3, 0.3)
            });
        });

        // label
        tile
    };

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

    // let label = Label::dynamic(|data: &AppState, _| format!("Origin: {:?}; {:?}", data.origin, data.message));
    let board = make_rows()
        .center()
        .padding(15.0)
        .background(Color::from_rgba32_u32(0x332205FF));

    // let board = Flex::column().with_flex_child(board, 1.0).padding((10.0, 0.0, 10.0, 0.0));

    Flex::column()
        // .with_child(label.padding(5.).center())
        .with_flex_child(board, 1.0)
}

struct Tile<T> {
    position: Position,
    piece: Option<Piece>,
    label: Label<T>,
}

impl<T: druid::Data> Tile<T> {
    fn new(position: Position, label: Label<T>) -> Self {
        // piece: Option<Piece>
        Self { position, piece: None, label }
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: druid::Data> Widget<T> for Tile<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.label.lifecycle(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        // self.label_size = 
        self.label.layout(ctx, &bc, data, env);

        bc.max()
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let pos = self.position;
        // let is_selected = data.origin.filter(|a| *a==pos).is_some();
        let checkerboard = pos.0 % 2 == pos.1 % 2;

        let bounds = ctx.size().to_rect();
        let is_active = ctx.is_active();
        let colo = ColorUtil::hsl(0.1, 0.2, if checkerboard { 0.1 } else { 0.5 } + if is_active {0.2} else {0.});

        ctx.fill(bounds, &colo);

        // self.label.paint(&mut ctx, c, env);
        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(Vec2::from((8.0, 8.0))));
            self.label.paint(ctx, data, env);
        });
    }
}

struct ColorUtil;
impl ColorUtil {
    pub fn hsl(h: f64, s: f64, l: f64) -> Color {
        Self::rbg8t(Self::hsl_to_rgb(h, s, l))
    }
    pub const fn rbg8t((r, g, b): (u8, u8, u8)) -> Color {
        Color::rgb8(r, g, b)
    }
    // https://pauljmiller.com/posts/druid-widget-tutorial.html
    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        let mut t = t;
        if t < 0. {
            t += 1.
        }
        if t > 1. {
            t -= 1.
        };
        if t < 1. / 6. {
            return p + (q - p) * 6. * t;
        }
        if t < 1. / 2. {
            return q;
        }
        if t < 2. / 3. {
            return p + (q - p) * (2. / 3. - t) * 6.;
        }
        return p;
    }

    fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let r;
        let g;
        let b;

        if s == 0.0 {
            r = l;
            g = l;
            b = l; // achromatic
        } else {
            let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };

            let p = 2. * l - q;
            r = Self::hue_to_rgb(p, q, h + 1. / 3.);
            g = Self::hue_to_rgb(p, q, h);
            b = Self::hue_to_rgb(p, q, h - 1. / 3.);
        }

        return (
            (r * 255.).round() as u8,
            (g * 255.).round() as u8,
            (b * 255.).round() as u8,
        );
    }
}
