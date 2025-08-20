use rusqlite::{self, Connection, Result};

use crate::ProcessInfo;

pub struct Sql {
    connection: Connection,
}

impl Sql {
    pub fn save(&mut self, data: &ProcessInfo) -> Result<()> {
        let start = data.start.format("%Y-%m-%d %H:%M:%S").to_string();
        let name = &data.name;
        self.connection.execute(
            "INSERT INTO ProcessInfo (start, procname) VALUES (?1, ?2)",
            (&start, name),
        )?;
        Ok(())
    }

    pub fn new() -> Result<Sql> {
        match connect() {
            Ok(conn) => Ok(Sql { connection: conn }),
            Err(e) => Err(e),
        }
    }
}

fn connect() -> Result<Connection> {
    let conn = Connection::open("db.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ProcessInfo (start TEXT, procname TEXT)", // Исправлено имя столбца
        (), // пустой список параметров.
    )?;

    Ok(conn)
}
