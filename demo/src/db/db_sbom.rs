use rusqlite::{Connection, Result, params};

/// Represents an SBOM entry
#[derive(Debug)]
pub struct Sbom {
    pub sbom: String,
    pub vendor: String,
    pub product: String,
    pub version: String,
}

/// Initialize the SBOM database
pub fn init_sbom_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Create the SBOM table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sbom (
            sbom TEXT NOT NULL,
            vendor TEXT NOT NULL,
            product TEXT NOT NULL,
            version TEXT NOT NULL,
            PRIMARY KEY (vendor, product, version)
        )",
        [],
    )?;

    Ok(conn)
}

/// Insert an SBOM entry into the SBOM database
pub fn insert_sbom(conn: &Connection, sbom: &Sbom) -> Result<()> {
    conn.execute(
        "INSERT INTO sbom (sbom, vendor, product, version) VALUES (?1, ?2, ?3, ?4)",
        &[&sbom.sbom, &sbom.vendor, &sbom.product, &sbom.version],
    )?;
    Ok(())
}


/// Query all SBOM entries from the SBOM database
pub fn get_all_sboms(conn: &Connection) -> Result<Vec<Sbom>> {
    let mut stmt = conn.prepare("SELECT sbom, vendor, product, version FROM sbom")?;
    let sboms = stmt.query_map([], |row| {
        Ok(Sbom {
            sbom: row.get(0)?,
            vendor: row.get(1)?,
            product: row.get(2)?,
            version: row.get(3)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(sboms)
}

fn create_connection() {
    let sbom_db_path = "./db/sbom.db";
    let sbom_conn = init_sbom_db(sbom_db_path);
}
