
#[test]
fn it_translates() {
    assert_eq!(
        maltebl_chess::chess_logic::translate_position('a', 4).unwrap(),
        (1, 4)
    )
}
