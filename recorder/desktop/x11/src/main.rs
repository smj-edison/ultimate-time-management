use std::iter::repeat;
use std::str;

use x11rb::connection::{Connection, RequestConnection};
use x11rb::protocol::xproto::AtomEnum;
use x11rb::protocol::xproto::{
    get_geometry, list_properties, query_tree, Window,
    get_atom_name
};
use x11rb::rust_connection::RustConnection;

use desktop::x11_util::{get_window_string_ascii, get_window_value, parse_c_string};
use desktop::atom_helper::build_atom_map;

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
            let name = String::from(atom.to_string()) + ": " + &parse_c_string(&name_raw);

            atom_names.push(name);
        }

        println!(
            "{}Window name: \x1B[1;32m{:?}\x1B[0m ({:?}), class: {:?} ({:?}), command: {:?} ({:?})",
            repeat_string,
            if let Some(name) = get_window_value(conn, child_window, 365 as u32, 350 as u32, 1024)? {
                String::from(str::from_utf8(&name)?)
            } else {
                String::from("")
            },
            /*get_window_string(conn, child_window, 39 as u32)?,*/ u32::from(AtomEnum::WM_NAME),
            get_window_string_ascii(conn, child_window, AtomEnum::WM_CLASS)?, u32::from(AtomEnum::WM_CLASS),
            get_window_string_ascii(conn, child_window, AtomEnum::WM_COMMAND)?, u32::from(AtomEnum::WM_COMMAND)
        );

        println!(
            "{}available atoms: {:?}",
            repeat_string,
            atom_names
        );        

        debug_window_info(conn, child_window, Some(depth.unwrap_or_else(|| 0) + 1))?;

        println!();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    let atoms = build_atom_map(&conn, 2048); // yes, 2048 is incredibly arbitrary, but yes, it works
    let net_wm_name = atoms.get(&"_NET_WM_NAME".to_string()).unwrap();
    let utf8_string = atoms.get(&"UTF8_STRING".to_string()).unwrap();

    println!("{:?}", utf8_string);

    debug_window_info(&conn, screen.root, None)?;

    Ok(())
}
