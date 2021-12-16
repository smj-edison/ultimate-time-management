use libc::c_char;
use std::ffi::CStr;
use std::iter::repeat;
use std::str;

use x11rb::connection::{Connection, RequestConnection};
use x11rb::protocol::xproto::AtomEnum;
use x11rb::protocol::xproto::{
    get_geometry, get_property, list_properties, query_tree, Atom, Window,
    get_atom_name, GetAtomNameReply
};
use x11rb::rust_connection::RustConnection;

fn parse_c_string(chars: &Vec<u8>) -> String {
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

fn get_window_string<Conn>(
    conn: &Conn,
    window: Window,
    string_to_get: AtomEnum,
) -> Result<Option<String>, Box<dyn std::error::Error>>
where
    Conn: RequestConnection + ?Sized,
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

fn debug_window_info<Conn>(
    conn: &Conn,
    window: Window,
    depth: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>>
where
    Conn: RequestConnection + ?Sized,
{
    let reply = query_tree(conn, window)?.reply()?;

    if !reply.children.is_empty() {
        println!("children: {}", reply.children.len());
    }

    for child_window in reply.children {
        let repeat_string = repeat(">>    ")
            .take(depth.unwrap_or_else(|| 0))
            .collect::<String>();

        // TODO: are all child windows themselves drawable?
        let window = get_geometry(conn, child_window)?.reply()?;
        let atoms = list_properties(conn, child_window)?.reply()?.atoms;
        let mut atom_names: Vec<String> = Vec::new();

        for atom in atoms {
            let name_raw = get_atom_name(conn, atom)?.reply()?.name;
            let name = parse_c_string(&name_raw);

            atom_names.push(name);
        }

        println!(
            "{}available atoms: {:?}",
            repeat_string,
            atom_names
        );
        println!(
            "{}Window name: {:?}, class: {:?}, command: {:?}",
            repeat_string,
            get_window_string(conn, child_window, AtomEnum::WM_NAME)?,
            get_window_string(conn, child_window, AtomEnum::WM_CLASS)?,
            get_window_string(conn, child_window, AtomEnum::WM_COMMAND)?,
        );

        debug_window_info(conn, child_window, Some(depth.unwrap_or_else(|| 0) + 1))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    debug_window_info(&conn, screen.root, None)?;

    Ok(())
}
