use libc::c_char;
use std::ffi::CStr;
use std::str;

use x11rb::protocol::xproto::{query_tree, get_geometry, list_properties, get_property, Window};
use x11rb::protocol::xproto::AtomEnum;
use x11rb::rust_connection::RustConnection;
use x11rb::connection::{Connection, RequestConnection};

fn get_window_string<Conn>(conn: &Conn, window: Window, string_to_get: AtomEnum) -> Result<Option<String>, Box<dyn std::error::Error>>
where Conn: RequestConnection + ?Sized {
    let window_prop = get_property(conn, false, window, string_to_get, AtomEnum::STRING, 0, 1024)?.reply()?;

    if window_prop.type_ != u32::from(AtomEnum::STRING) {
        return Ok(None);
    }
    
    let str = if window_prop.value[(window_prop.length - 1) as usize] == 0x00 {
        String::from(CStr::from_bytes_with_nul(&window_prop.value)?.to_str()?.clone())
    } else {
        String::from(window_prop.value.iter().map(|&x| x as char))
    }


    Ok(Some(str))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;

    let screen = &conn.setup().roots[screen_num];
    
    let reply = query_tree(&conn, screen.root)?.reply()?;

    let blank = String::from("");

    for child_window in reply.children {
        // TODO: are all child windows themselves drawable?
        let window = get_geometry(&conn, child_window)?.reply()?;
        //let atoms = list_properties(&conn, child_window)?.reply()?.atoms;        

        println!(
            "Window name: {:?}, class: {:?}, command: {:?}",
            get_window_string(&conn, child_window, AtomEnum::WM_NAME)?,
            get_window_string(&conn, child_window, AtomEnum::WM_CLASS)?,
            get_window_string(&conn, child_window, AtomEnum::WM_COMMAND)?
        );
    }

    Ok(())
}