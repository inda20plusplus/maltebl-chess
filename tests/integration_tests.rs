use maltebl_chess::{chess_logic::*, console_display::*, piece_logic::*};
#[test]
fn it_translates() {
    assert_eq!(to_coords('a', 4).unwrap(), (1, 4));
    assert_eq!(to_notation((1, 4)).unwrap(), ('a', 4));
}
#[test]
#[should_panic(expected = "Tried to add piece at non-empty space at (0, 0)")]
fn occupied_spot() {
    let mut board: ChessBoard = init_board();
    print_board(board.ref_board());
    board.add_piece(piece_make(Color::Black, PieceType::Pawn, (0, 0)));
}

#[test]
fn move_piece() {
    let mut board: ChessBoard = init_board();
    print_board(board.ref_board());
    board.move_piece((1, 1), (1, 2));
    print_board(board.ref_board());
    board.move_piece((1, 2), (1, 0));
    print_board(board.ref_board());
    board.move_piece((1, 0), (1, 7));
    print_board(board.ref_board());
}

#[test]
fn check_move() {
    let mut board: ChessBoard = init_board();
    print_board(board.ref_board());
    println!("{:?}", board.legal_moves(board.ref_piece((1, 0)).unwrap()));
    board.move_piece((3, 1), (6, 3));
    board.move_piece((6, 6), (6, 4));
    print_board(board.ref_board());
    println!("{:?}", board.legal_moves(board.ref_piece((2, 0)).unwrap()));
}
