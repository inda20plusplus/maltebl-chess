use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod color_util;
mod delegate;
mod state;
mod tile;
mod ui;

use crate::delegate::{action, Delegate};
use crate::state::AppState;
use crate::ui::main_ui;

use maltebl_chess::{chess_game::*, piece_logic::Color, to_coords, to_notation};
use tcp_chess_com::connection_lib as con;

use clap::Clap;
use druid::{AppLauncher, Env, ExtEventSink, PlatformError, WindowDesc};

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Leonard Pauli and Malte Blomqvist; indaplusplus"
)]
struct Opts {
    /// used with net_connect (black), ie. --net_listen=0.0.0.0:8032
    #[clap(long)]
    net_listen: Option<String>,
    /// used with net_listen (white, first move), ie. --net_connect=0.0.0.0:8032
    #[clap(long)]
    net_connect: Option<String>,
}

/**

(networking extention created by Leonard Pauli, autum 2020, inda)
 (1s delay intentionally used to show async:ness)
Data flow overview:

white
    delegate
        on ui_move: or game.validate_move:
            - ok:
                - do game.move (inc gui.update)
                - do net.send_move
            - err: show
    network
        on send_move: do other.receive_move
        on send_move_ok: do other.receive_move_ok
        on send_move_decline: do other.receive_move_decline

        on receive_move: or game.validate_move:
            - ok:
                - do game.move (inc gui.update)
                - do net.send_move_ok
            - err: do net.send_move_decline
        on receive_move_ok: noop
        on receive_move_decline: undo relevant move (inc gui.update)
    gui
        on move: do delegate.ui_move

white.network <-(>) black.network // black connects to white listener with tcp
    tcp stream read/write
gui (<)-> delegate // gui submits commands to delegate, which may change data gui uses
    -> ctx.submit_command
    <- mutate data
net (<)-> delegate // net may ask delegate to validate move, which responds back later
    -> event_sink.submit_command
    <- net_sender/delegate_tx.send

net <-> tcp // (unnecessary?) intermediate for logic
    -> tcp_tx
    <- tcp_rx

*/

fn main() -> Result<(), PlatformError> {
    let opts: Opts = Opts::parse();
    let networked = opts.net_connect.is_some() || opts.net_listen.is_some();

    let game = init_standard_chess();
    let data = AppState::new(game.get_board());

    let (delegate_tx, delegate_rx) = mpsc::channel();
    let single_player = match networked {
        false => None,
        true => Some(match opts.net_listen.is_some() {
            true => Color::White,
            false => Color::Black,
        }),
    };
    let net_sender = match networked {
        false => None,
        true => Some(delegate_tx),
    };
    let delegate = Delegate {
        game,
        single_player,
        net_sender,
    };

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
    let event_sink = app.get_external_handle();

    if networked {
        println!("Networked.");
        setup_networking(&opts, event_sink, delegate_rx);
    }

    app.delegate(delegate).launch(data)?;

    Ok(())
}

fn setup_networking(opts: &Opts, event_sink: ExtEventSink, delegate_rx: mpsc::Receiver<String>) {
    let (net_tx, net_rx) = mpsc::channel::<con::Message>();

    if let Some(addr) = opts.net_connect.as_ref() {
        let addr = addr.clone();
        thread::spawn(move || {
            println!("Connecting to {}", addr);

            let mut stream = TcpStream::connect(addr).unwrap();
            println!("Connection established.");
            println!("Awaiting remote's players initial move.");

            loop {
                if let Some(err) = await_receive_move(&mut stream, &event_sink, &net_rx).err() {
                    println!("ERR: {:?}", err);
                    break;
                };
                if let Some(err) = await_transmit_move(&mut stream, &event_sink, &net_rx).err() {
                    println!("ERR: {:?}", err);
                    break;
                };
            }
        });
    } else {
        let addr = opts.net_listen.as_ref().unwrap();
        let addr = addr.clone();
        thread::spawn(move || {
            let listener = TcpListener::bind(&addr).unwrap();
            println!("Awaiting connection at {}", addr);

            for stream in listener.incoming() {
                // TODO: a lot to improve; stability, clean code, feature set, ...

                let mut stream = stream.unwrap();
                println!("Connection established.");
                println!("Make a move as initial message.");

                loop {
                    if let Some(err) = await_transmit_move(&mut stream, &event_sink, &net_rx).err()
                    {
                        println!("ERR: {:?}", err);
                        break;
                    };

                    if let Some(err) = await_receive_move(&mut stream, &event_sink, &net_rx).err() {
                        println!("ERR: {:?}", err);
                        break;
                    };
                }
            }
        });
    }

    // input from delegate
    thread::spawn(move || {
        for received in delegate_rx {
            println!("from delegate: {:?}", received);
            thread::sleep(Duration::from_secs(1));

            let message = match &received[..] {
                "ok" => con::Message::Accept,
                _ => {
                    // TODO: error handling
                    let mut parts = received
                        .split(' ')
                        .map(|a| to_coords(a.to_owned()).unwrap())
                        .map(|(x, y)| con::Position::new(x as u8, y as u8));
                    let origin = parts.next().unwrap();
                    let target = parts.next().unwrap();

                    con::Message::Move(con::Move::Standard { origin, target })
                }
            };

            net_tx.send(message).unwrap();
        }
    });
}

enum ResponseKind {
    Noop,
    Message(con::Message),
    AwaitDelegate,
}

// TODO: clean up message order flow to accept any order
//  currently will be in an inconsistent/locked state if unexpected order

fn await_transmit_move(
    stream: &mut TcpStream,
    _event_sink: &ExtEventSink,
    net_rx: &mpsc::Receiver<con::Message>,
) -> Result<(), String> {
    let message = net_rx.recv().unwrap();
    println!("Message: {:?}", message);
    let message = message.code();
    stream.write_all(&message[..]).unwrap();
    stream.flush().unwrap();

    let mut buffer = [0; 32];
    stream.read(&mut buffer).unwrap();

    let message = con::Message::from_code(&buffer[..]);

    let response_kind = {
        println!("Received: {:?}", message);

        match message {
            con::Message::Accept => ResponseKind::Noop,
            con::Message::Decline => unimplemented!(),
            _ => {
                println!("Message out of order");
                ResponseKind::Message(con::Message::Decline)
            }
        }
    };

    let response = match response_kind {
        ResponseKind::Noop => None,
        ResponseKind::Message(msg) => Some(msg),
        ResponseKind::AwaitDelegate => {
            let response = net_rx.recv().unwrap();
            Some(response)
        }
    };
    if let Some(response) = response {
        println!("Response: {:?}", response);
        let response = response.code();
        stream.write_all(&response[..]).unwrap();
        stream.flush().unwrap();
    }
    Ok(())
}

fn await_receive_move(
    stream: &mut TcpStream,
    event_sink: &ExtEventSink,
    net_rx: &mpsc::Receiver<con::Message>,
) -> Result<(), String> {
    let mut buffer = [0; 32];
    stream.read(&mut buffer).unwrap();

    let message = con::Message::from_code(&buffer[..]);
    let response_kind = {
        println!("Received: {:?}", message);

        match message {
            con::Message::Accept => ResponseKind::Noop,
            con::Message::Decline => ResponseKind::Noop,
            con::Message::Move(con::Move::Standard { origin, target }) => {
                // TODO: error handling, remote should not be able to crash client
                let origin = to_notation((origin.x as usize, origin.y as usize)).unwrap();
                let target = to_notation((target.x as usize, target.y as usize)).unwrap();
                let command = format!("{} {}", origin, target);

                let res = event_sink.submit_command(action::MAKE_MOVE_FROM_NET, command, None);
                if let Some(err) = res.err() {
                    return Err(err.to_string());
                }

                // Accept/Decline is sent from delegate
                ResponseKind::AwaitDelegate
            }
            _ => ResponseKind::Message(con::Message::Decline),
        }
    };

    let response = match response_kind {
        ResponseKind::Noop => None,
        ResponseKind::Message(msg) => Some(msg),
        ResponseKind::AwaitDelegate => {
            let response = net_rx.recv().unwrap();
            Some(response)
        }
    };
    if let Some(response) = response {
        println!("Response: {:?}", response);
        let response = response.code();
        stream.write_all(&response[..]).unwrap();
        stream.flush().unwrap();
    }
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
