use rusqlite::{Connection, Result};

pub fn check_database() -> Result<()> {
    let connection = Connection::open("ur_save.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS ur_save (
            id INTEGER PRIMARY KEY,
            name TEXT,
            url TEXT
        )", []);

    Ok(())
}
