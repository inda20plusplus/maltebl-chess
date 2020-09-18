use crate::{board_logic::*, console_display::*, *};
#[test]
fn it_translates() {
    assert_eq!(to_coords("a4".to_string()).unwrap(), (1, 4));
    assert_eq!(to_notation((1, 4)).unwrap(), ("a4"));
}
#[test]
#[should_panic(expected = "Tried to add piece at non-empty space at (0, 0)")]
fn occupied_spot() {
    let mut board: ChessBoard = init_board();
    board.standard_pieces(Color::White);
    board.standard_pieces(Color::Black);
    print_board(board.ref_board());
    board.add_piece(piece_make(Color::Black, PieceType::Pawn), (0, 0));
}

#[test]
fn pawn_moves() {
    let mut board: ChessBoard = init_board();
    board.standard_pieces(Color::White);
    board.standard_pieces(Color::Black);
    board.force_move((4, 6), (4, 2)).expect("force_move panic");
    print_board(board.ref_board());
    let mut moves = board.regular_moves((3, 1));
    moves.push(board.special_moves((3, 1)).pop().unwrap().0);
    assert_eq!(moves, vec![(3, 2), (4, 2), (3, 3)])
}

#[test]
fn passant() {
    let mut board: ChessBoard = init_board();
    board.standard_pieces(Color::White);
    board.standard_pieces(Color::Black);
    board.force_move((4, 6), (4, 3)).expect("force_move panic");
    print_board(board.ref_board());
    board.move_piece((3, 1), ((3, 3), Some(SpecialMove::Pawn2Step)));
    print_board(board.ref_board());
    for mov in board.get_moves((4, 3)) {
        println!("{:?}", mov.0);
    }
    board.move_piece((4, 3), ((3, 2), None));
    print_board(board.ref_board());
}
#[test]
fn continous_moves() {
    let mut board: ChessBoard = init_board();
    board.standard_pieces(Color::White);
    board.standard_pieces(Color::Black);
    board.force_move((3, 1), (3, 3)).expect("force_move panic");
    board.force_move((3, 0), (3, 1)).expect("force_move panic");
    print_board(board.ref_board());
    let mut queen_moves = vec![
        (0, 4),
        (1, 3),
        (2, 2),
        (3, 2),
        (3, 0),
        (4, 2),
        (5, 3),
        (6, 4),
        (7, 5),
    ];
    queen_moves.sort();
    let mut moves = board.regular_moves((3, 1));
    moves.sort();
    assert_eq!(queen_moves, moves)
}
#[test]
fn castling() {
    let mut board: ChessBoard = init_board();
    board.add_piece(piece_make(Color::White, PieceType::King), (4, 0));
    board.add_piece(piece_make(Color::White, PieceType::Knight), (3, 0));
    board.add_piece(piece_make(Color::White, PieceType::Pawn), (3, 1));
    board.add_piece(piece_make(Color::White, PieceType::Rook), (7, 0));
    board.add_piece(piece_make(Color::White, PieceType::Rook), (0, 0));
    board.standard_pieces(Color::Black);
    board.force_move((0, 7), (3, 5)).expect("force_move panic");
    print_board(board.ref_board());
    assert_eq!(board.special_moves((4, 0)).len(), 1);
    board.force_move((3, 0), (6, 1)).expect("force_move panic");
    print_board(board.ref_board());
    assert_eq!(board.special_moves((4, 0)).len(), 2);
    board.force_move((3, 1), (6, 0)).expect("force_move panic");
    print_board(board.ref_board());
    assert_eq!(board.special_moves((4, 0)).len(), 0);
    board.force_move((6, 0), (6, 2)).expect("force_move panic");
    print_board(board.ref_board());
    assert_eq!(board.special_moves((4, 0)).len(), 1);
}
#[test]
fn finds_checked() {
    let mut board: ChessBoard = init_board();
    board.add_piece(piece_make(Color::White, PieceType::King), (3, 0));
    board.add_piece(piece_make(Color::Black, PieceType::Rook), (3, 3));
    print_board(board.ref_board());
    assert_eq!(true, board.is_checked(Color::White))
}
#[test]
fn self_check() {
    let mut board: ChessBoard = init_board();
    board.add_piece(piece_make(Color::White, PieceType::King), (3, 0));
    board.add_piece(piece_make(Color::White, PieceType::Knight), (3, 1));
    board.add_piece(piece_make(Color::Black, PieceType::Pawn), (1, 2));
    board.add_piece(piece_make(Color::Black, PieceType::Rook), (3, 3));
    print_board(board.ref_board());
    assert_eq!(0, board.get_moves((3, 1)).len())
}

#[test]
fn check_mate() {
    let mut board: ChessBoard = init_board();
    board.add_piece(piece_make(Color::White, PieceType::King), (7, 7));
    board.add_piece(piece_make(Color::Black, PieceType::Knight), (5, 5));
    board.add_piece(piece_make(Color::Black, PieceType::Rook), (7, 6));
    print_board(board.ref_board());
    assert_eq!(true, board.self_check((7, 7), (6, 6)));
    assert_eq!(true, board.is_checkmate(Color::White));
    let mut board2: ChessBoard = init_board();
    board2.standard_pieces(Color::Black);
    board2.standard_pieces(Color::White);
    board2.force_move((5, 1), (5, 2)).expect("force_move panic");
    board2.force_move((4, 6), (4, 4)).expect("force_move panic");
    print_board(board2.ref_board());
    assert_eq!(false, board2.is_checkmate(Color::White));
    board2.force_move((6, 1), (6, 3)).expect("force_move panic");
    board2.force_move((3, 7), (7, 3)).expect("force_move panic");
    print_board(board2.ref_board());
    assert_eq!(true, board2.is_checkmate(Color::White))
}
