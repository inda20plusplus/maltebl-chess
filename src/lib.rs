pub mod board_logic;
pub mod console_display;
pub mod piece_logic;

pub fn to_coords(input: String) -> Result<(usize, usize), String> {
    if input.len() == 2 {
        let mut input = input.chars();
        let pos_x = input.next().unwrap() as isize - 96;
        let pos_y = input.next().unwrap().to_string().parse().unwrap();
        if pos_x < 0 || pos_x > 7 || pos_y > 7 {
            println!("{}{}", pos_x, pos_y);
            return Err(String::from("tried to access non-existent boardspace"));
        }
        Ok((pos_x as usize, pos_y))
    } else {
        Err(String::from(
            "invalid notation, cannot find coords on board",
        ))
    }
}

pub fn to_notation(position: (usize, usize)) -> Result<String, String> {
    let (x, y) = position;
    if x < 1 || x > 8 || y < 1 || y > 8 {
        return Err(String::from("tried to access non-existent boardspace"));
    }
    Ok(format!("{}{}", (x + 96) as u8 as char, y))
}
