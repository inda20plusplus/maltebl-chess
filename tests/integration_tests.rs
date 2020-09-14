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
    board.add_piece(piece_make(Color::Black, PieceType::Pawn), (0, 0));
}
#[test]
fn pawn_moves() {
    let mut board: ChessBoard = init_board();
    board.move_illegal((3, 1), (3, 3));
    board.move_illegal((4, 6), (4, 4));
    print_board(board.ref_board());
    println!("{:?}", board.legal_moves((3, 3)));
}
#[test]
fn continous_moves() {
    let mut board: ChessBoard = init_board();
    board.move_illegal((3, 1), (3, 3));
    print_board(board.ref_board());
    println!("{:?}", board.legal_moves((2, 0)));
}
