use log::{info, error};

mod cli {
    pub mod cli;
}

use cli::{
    cli::build_cli,
};

mod db {
    pub mod db_commitment;
    pub mod db_sbom;
}

use db::{
    db_commitment::{init_commitment_db, insert_commitment, get_all_commitments, Commitment},
    db_sbom::{init_sbom_db, insert_sbom, get_all_sboms, Sbom},
};

mod commands {
    pub mod get_commitment;
    pub mod get_zk_proof;
    pub mod upload;
    pub mod verify;
}

use commands::{
    get_commitment::get_commitment,
    get_zk_proof::get_zk_proof,
    upload::upload_sbom,
    verify::verify,
};

fn main() {
    // Initialize the logger
    env_logger::init();
    info!("Starting the application...");

    // Clear databases
    let mut i = clear_databases();
    info!("Databases cleared: {:?}", i);

    // Initialize databases
    i = init_databases();
    info!("Databases initialized: {:?}", i);

    // Insert test data
    i = test_db_inserts();
    info!("Test db inserts: {:?}", i);

    // Build the CLI parser
    let matches = build_cli().get_matches();

    // Match the subcommand and call the corresponding function
    match matches.subcommand() {
        Some(("upload", sub_matches)) => {
            let sbom_path = sub_matches.get_one::<String>("sbom").unwrap();
            let vendor = sub_matches.get_one::<String>("vendor").unwrap();
            let product = sub_matches.get_one::<String>("product").unwrap();
            let version = sub_matches.get_one::<String>("version").unwrap();
            upload_sbom(sbom_path, vendor, product, version);
        }
        Some(("get-commitment", sub_matches)) => {
            let product = sub_matches.get_one::<String>("product").unwrap();
            let version = sub_matches.get_one::<String>("version").unwrap();
            get_commitment(product, version);
        }
        Some(("get-zk-proof", sub_matches)) => {
            let api_key = sub_matches.get_one::<String>("api-key").unwrap();
            let commitment = sub_matches.get_one::<String>("commitment").unwrap();
            let vulnerability = sub_matches.get_one::<String>("vulnerability").unwrap();
            get_zk_proof(api_key, commitment, vulnerability);
        }
        Some(("verify", sub_matches)) => {
            let commitment = sub_matches.get_one::<String>("commitment").unwrap();
            let zkproof = sub_matches.get_one::<String>("zkproof").unwrap();
            verify(commitment, zkproof);
        }
        _ => {
            error!("No valid subcommand provided. Use --help for usage information.");
        }
    }
}

fn init_databases() -> Result<(), Box<dyn std::error::Error>> {
    let sbom_db_path = "./db/sbom.db";
    let commitment_db_path = "./db/commitment.db";

    let sbom_conn = init_sbom_db(sbom_db_path)?;
    let commitment_conn = init_commitment_db(commitment_db_path)?;

    let sboms = get_all_sboms(&sbom_conn)?;
    info!("SBOMs: {:?}", sboms);

    let commitments = get_all_commitments(&commitment_conn)?;
    info!("Commitments: {:?}", commitments);

    Ok(())
}

fn test_db_inserts() -> Result<(), Box<dyn std::error::Error>> {
    let sbom_db_path = "./db/sbom.db";
    let commitment_db_path = "./db/commitment.db";

    let sbom_conn = init_sbom_db(sbom_db_path)?;
    let commitment_conn = init_commitment_db(commitment_db_path)?;

    let sbom = Sbom {
        sbom: "example_sbom".to_string(),
        vendor: "example_vendor".to_string(),
        product: "example_product".to_string(),
        version: "1.1".to_string(),
    };

    let commitment = Commitment {
        vendor: "example_vendor".to_string(),
        product: "example_product".to_string(),
        version: "1.1".to_string(),
        commitment: "example_commitment".to_string(),
    };

    insert_sbom(&sbom_conn, &sbom)?;
    insert_commitment(&commitment_conn, &commitment)?;

    let sboms = get_all_sboms(&sbom_conn)?;
    info!("SBOMs: {:?}", sboms);

    let commitments = get_all_commitments(&commitment_conn)?;
    info!("Commitments: {:?}", commitments);

    Ok(())
}

fn clear_databases() -> Result<(), Box<dyn std::error::Error>> {
    let sbom_db_path = "./db/sbom.db";
    let commitment_db_path = "./db/commitment.db";

    let sbom_conn = init_sbom_db(sbom_db_path)?;
    let commitment_conn = init_commitment_db(commitment_db_path)?;

    sbom_conn.execute("DELETE FROM sbom", [])?;
    commitment_conn.execute("DELETE FROM commitment", [])?;

    Ok(())
}
