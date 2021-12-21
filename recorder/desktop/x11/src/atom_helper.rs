use std::iter::Map;
use std::collections::HashMap;

use x11rb::connection::{RequestConnection};
use x11rb::protocol::xproto::{
    get_atom_name
};

use crate::x11_util::parse_c_string;

/// Creates a hashmap of atom names to atom values.
/// I can't be confident that the atom names will be the same across OSes,
/// So I opted to assume they could be different
pub fn build_atom_map<Conn>(
    conn: &Conn,
    search_size: u32
) -> HashMap<String, u32>
where
    Conn: RequestConnection + ?Sized
{
    let mut atoms: HashMap<String, u32> = HashMap::new();

    for atom in 0..search_size {
        let name_raw = match get_atom_name(conn, atom) {
            Ok(result) =>  match result.reply() {
                Ok(result) => result.name,
                Err(_) => continue
            },
            Err(_) => continue
        };

        let name = String::from(&parse_c_string(&name_raw));

        atoms.insert(name, atom);
    }

    atoms
}