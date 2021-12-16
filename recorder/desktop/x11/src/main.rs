use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::iter::repeat;

use x11rb::protocol::xproto::{query_tree, get_geometry, list_properties, get_property, Window, Atom};
use x11rb::protocol::xproto::AtomEnum;
use x11rb::rust_connection::RustConnection;
use x11rb::connection::{Connection, RequestConnection};

fn get_window_string<Conn>(conn: &Conn, window: Window, string_to_get: AtomEnum) -> Result<Option<String>, Box<dyn std::error::Error>>
where Conn: RequestConnection + ?Sized {
    let window_prop = get_property(conn, false, window, string_to_get, AtomEnum::STRING, 0, 1024)?.reply()?;

    if window_prop.type_ != u32::from(AtomEnum::STRING) {
        return Ok(None);
    }
    
    let converted_str: String = window_prop.value.iter().map(|&x| x as char).collect();

    // deal with null terminators (for some reason they only sometimes show?)
    let possibly_terminator_index = converted_str.find('\0');

    let str_no_terminator = if let Some(terminator_index) = possibly_terminator_index {
        &converted_str[0..terminator_index]
    } else {
        &converted_str
    };


    Ok(Some(String::from(str_no_terminator)))
}

fn debug_window_info<Conn>(conn: &Conn, window: Window, depth: Option<usize>) -> Result<(), Box<dyn std::error::Error>>
where Conn: RequestConnection + ?Sized {
    let reply = query_tree(conn, window)?.reply()?;

    if !reply.children.is_empty() {
        println!("children: {}", reply.children.len());
    }

    for child_window in reply.children {
        let repeat_string = repeat(">>    ").take(depth.unwrap_or_else(|| 0)).collect::<String>();

        // TODO: are all child windows themselves drawable?
        let window = get_geometry(conn, child_window)?.reply()?;
        let atoms = list_properties(conn, child_window)?.reply()?.atoms;        

        //println!("available atoms: {:?}", atoms.iter().map(|&x| AtomEnum::from(x as u8)).collect::<Vec<AtomEnum>>());
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