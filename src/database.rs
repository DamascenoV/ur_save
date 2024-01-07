use crate::models::Url;
use rusqlite::{Connection, Result};

pub fn check_database() -> Result<()> {
    let connection = Connection::open("ur_save.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS ur_save (
            id INTEGER PRIMARY KEY,
            name TEXT,
            url TEXT
        )",
        [],
    )?;

    Ok(())
}

pub fn insert(url: &Url) -> Result<()> {
    let connection = Connection::open("ur_save.db")?;

    connection.execute(
        "INSERT INTO ur_save (name, url) VALUES (?1, ?2)",
        [&url.name, &url.url],
    )?;

    let mut stmt = connection.prepare("SELECT id, name, url FROM ur_save")?;
    let rows = stmt.query_map([], |row| {
        Ok(Url {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
        })
    })?;

    for row in rows {
        println!("Found: {:?}", row.unwrap());
    }

    Ok(())
}

pub fn get_all() -> Result<Vec<Url>> {
    let connection = Connection::open("ur_save.db")?;

    let mut urls = Vec::new();
    let mut stmt = connection.prepare("SELECT id, name, url FROM ur_save")?;
    let rows = stmt.query_map([], |row| {
        Ok(Url {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
        })
    })?;

    for row in rows {
        urls.push(row.unwrap());
    }

    Ok(urls)
}

pub fn get_by_name(name: String) -> Result<Url> {
    let connection = Connection::open("ur_save.db")?;

    let mut stmt =
        connection.prepare("SELECT id, name, url FROM ur_save WHERE name = ?1 LIMIT 1")?;
    let rows = stmt.query_map([&name], |row| {
        Ok(Url {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
        })
    })?;

    for row in rows {
        return Ok(row.unwrap());
    }

    return Err(rusqlite::Error::QueryReturnedNoRows);
}

pub fn delete_by_name(name: String) -> Result<()> {
    let connection = Connection::open("ur_save.db")?;

    let row = connection.execute("DELETE FROM ur_save WHERE name = ?1", [&name])?;

    if row == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    Ok(())
}

pub fn update(url: &Url) -> Result<()> {
    let connection = Connection::open("ur_save.db")?;

    let row = connection.execute(
        "UPDATE ur_save SET name = ?1, url = ?2 WHERE id = ?3",
        [&url.name, &url.url, &url.id.to_string()],
    )?;

    if row == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    Ok(())
}
