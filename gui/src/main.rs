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

// use druid::{
//     widget::{FillStrat, Flex, Svg, SvgData, WidgetExt},
//     AppLauncher, LocalizedString, Widget, WindowDesc,
// };

// pub fn main() {
//     let main_window = WindowDesc::new(ui_builder)
//         .title(LocalizedString::new("svg-demo-window-title").with_placeholder("Rawr!"));
//     let data = 0_u32;
//     AppLauncher::with_window(main_window)
//         .use_simple_logger()
//         .launch(data)
//         .expect("launch failed");
// }

// fn ui_builder() -> impl Widget<u32> {
//     let tiger_svg = match include_str!("../assets/tiger.svg").parse::<SvgData>() {
//         Ok(svg) => svg,
//         Err(err) => {
//             panic!("{}", err);
//             // SvgData::default()
//         }
//     };
//     Svg::new(tiger_svg.clone()).fix_width(60.0).center()

//     let mut col = Flex::column();

//     col.add_flex_child(Svg::new(tiger_svg.clone()).fix_width(60.0).center(), 1.0);
//     col.add_flex_child(Svg::new(tiger_svg.clone()).fill_mode(FillStrat::Fill), 1.0);
//     col.add_flex_child(Svg::new(tiger_svg), 1.0);
//     col.debug_paint_layout()
// }
