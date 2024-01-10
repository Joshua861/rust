use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Todo {
    content: String,
    done: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
}
