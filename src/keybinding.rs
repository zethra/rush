///Key
///Enum used for Key bindings on the keyboard
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Del,
    Home,
    End,
    Tab,
    Char(char),
    Null, // This if for when nothing needs to be returned to signify no action needs to be taken and can't be made with new_key
    Esc,
}

///New Key
///Used to create a new key from an integer input.
///
///Primarily used by the InputBuffer struct whil recieving input from
///the Command Line. If needed it's better just use the enum directly
///rather than this function
pub fn new_key(input: i32) -> Key {
    match input {
        -1 => Key::Up,
        -2 => Key::Down,
        -3 => Key::Right,
        -4 => Key::Left,
        -5 => Key::Enter,
        -6 => Key::Backspace,
        -7 => Key::Del,
        -8 => Key::Home,
        -9 => Key::End,
        -10 => Key::Tab,
        -11 => Key::Esc,
        _ => Key::Char(input as u8 as char),
    }
}
