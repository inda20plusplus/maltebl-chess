pub mod board_logic;
pub mod console_display;
pub mod piece_logic;

pub fn to_coords(input: String) -> Result<(usize, usize), String> {
    if input.len() == 2 {
        let mut input = input.chars();
        let mut pos_x = input.next().unwrap() as isize - 96;
        let mut pos_y: isize = input.next().unwrap().to_string().parse().unwrap();
        pos_x -= 1;
        pos_y -= 1;
        if pos_x < 0 || pos_x > 7 || pos_y < 0 || pos_y > 7 {
            println!("{}{}", pos_x, pos_y);
            return Err(String::from("tried to access non-existent boardspace"));
        }
        Ok((pos_x as usize, pos_y as usize))
    } else {
        Err(String::from(
            "invalid notation, cannot find coords on board",
        ))
    }
}

pub fn to_notation(position: (usize, usize)) -> Result<String, String> {
    let (x, y) = position;
    if x > 8 || y > 8 {
        return Err(String::from("tried to access non-existent boardspace"));
    }
    Ok(format!("{}{}", (x + 97) as u8 as char, y + 1))
}
