use rusqlite::{Connection, Result, params};

/// Represents a Commitment entry
#[derive(Debug)]
pub struct Commitment {
    pub vendor: String,
    pub product: String,
    pub version: String,
    pub commitment: String,
}

/// Initialize the Commitment database
pub fn init_commitment_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Create the Commitment table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commitment (
            vendor TEXT NOT NULL,
            product TEXT NOT NULL,
            version TEXT NOT NULL,
            commitment TEXT NOT NULL,
            PRIMARY KEY (vendor, product, version)
        )",
        [],
    )?;

    Ok(conn)
}

/// Insert a Commitment entry into the Commitment database
pub fn insert_commitment(conn: &Connection, commitment: &Commitment) -> Result<()> {
    conn.execute(
        "INSERT INTO commitment (vendor, product, version, commitment) VALUES (?1, ?2, ?3, ?4)",
        params![commitment.vendor, commitment.product, commitment.version, commitment.commitment],
    )?;
    Ok(())
}

/// Query all Commitment entries from the Commitment database
pub fn get_all_commitments(conn: &Connection) -> Result<Vec<Commitment>> {
    let mut stmt = conn.prepare("SELECT vendor, product, version, commitment FROM commitment")?;
    let commitments = stmt.query_map([], |row| {
        Ok(Commitment {
            vendor: row.get(0)?,
            product: row.get(1)?,
            version: row.get(2)?,
            commitment: row.get(3)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(commitments)
}
