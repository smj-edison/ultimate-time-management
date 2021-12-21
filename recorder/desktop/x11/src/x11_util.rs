use x11rb::connection::RequestConnection;
use x11rb::protocol::xproto::AtomEnum;
use x11rb::protocol::xproto::{
    get_property, Atom, Window,
};

pub fn parse_c_string(chars: &Vec<u8>) -> String {
    let converted_str: String = chars.iter().map(|&x| x as char).collect();

    // deal with null terminators (for some reason they only sometimes show?)
    let possibly_terminator_index = converted_str.find('\0');

    let str_no_terminator = if let Some(terminator_index) = possibly_terminator_index {
        &converted_str[0..terminator_index]
    } else {
        &converted_str
    };

    String::from(str_no_terminator)
}

pub fn get_window_value<Conn, A>(
    conn: &Conn,
    window: Window,
    value_to_get: A,
    value_type: u32,
    value_length: u32
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Atom>
{
    let window_prop = get_property(
        conn,
        false,
        window,
        value_to_get,
        value_type,
        0,
        value_length,
    )?
    .reply()?;

    if window_prop.type_ != u32::from(value_type) {
        return Ok(None);
    }

    Ok(Some(window_prop.value))
}

pub fn get_window_string_ascii<Conn, A>(
    conn: &Conn,
    window: Window,
    string_to_get: A,
) -> Result<Option<String>, Box<dyn std::error::Error>>
where
    Conn: RequestConnection + ?Sized,
    A: Into<Atom>
{
    let window_prop = get_property(
        conn,
        false,
        window,
        string_to_get,
        AtomEnum::STRING,
        0,
        1024,
    )?
    .reply()?;

    if window_prop.type_ != u32::from(AtomEnum::STRING) {
        return Ok(None);
    }

    Ok(Some(parse_c_string(&window_prop.value)))
}
