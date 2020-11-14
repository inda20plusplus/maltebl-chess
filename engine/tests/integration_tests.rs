use maltebl_chess::chess_game::*;
#[test]
fn test_chessgame() {
    let mut chess = init_standard_chess();
    chess.move_piece("f2 f3".to_string()).unwrap();
    chess.move_piece("e7 e5".to_string()).unwrap();
    chess.move_piece("g2 g4".to_string()).unwrap();
    assert_eq!(
        true,
        chess
            .move_piece("d8 h4".to_string())
            .unwrap()
            .contains("checkmate")
    );
    chess.print_board();
}

#[test]
#[should_panic(expected = "That is not your piece!")]
fn test_turns() {
    let mut chess = init_standard_chess();
    chess.move_piece("f2 f3".to_string()).unwrap();
    chess.move_piece("g2 g4".to_string()).unwrap();
}

#[test]
fn pick_piece() {
    let chess = init_standard_chess();
    assert_eq!(
        vec!["f3", "f4"],
        chess.pick_piece("f2".to_string()).unwrap()
    )
}
