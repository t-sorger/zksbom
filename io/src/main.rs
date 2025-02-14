use clap::{Arg, Command};
use log::{error};

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

    // Define the CLI structure
    let matches = Command::new("ZK-SBOM Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A tool for managing SBOMs and ZK proofs")
        .subcommand(
            Command::new("upload")
                .about("Upload or update an SBOM")
                .arg(
                    Arg::new("sbom")
                        .long("sbom")
                        .value_name("FILE")
                        .help("Path to the SBOM file")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get-commitment")
                .about("Get the commitment for a product and version")
                .arg(
                    Arg::new("product")
                        .long("product")
                        .value_name("PRODUCT")
                        .help("Name of the product")
                        .required(true),
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .value_name("VERSION")
                        .help("Version of the product")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get-zk-proof")
                .about("Get a ZK proof for a given commitment and vulnerability")
                .arg(
                    Arg::new("api-key")
                        .long("api-key")
                        .value_name("API_KEY")
                        .help("API key for authentication")
                        .required(true),
                )
                .arg(
                    Arg::new("commitment")
                        .long("commitment")
                        .value_name("COMMITMENT")
                        .help("The commitment hash")
                        .required(true),
                )
                .arg(
                    Arg::new("vulnerability")
                        .long("vulnerability")
                        .value_name("VULNERABILITY")
                        .help("The vulnerability identifier")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify a ZK proof against a commitment")
                .arg(
                    Arg::new("commitment")
                        .long("commitment")
                        .value_name("COMMITMENT")
                        .help("The commitment hash")
                        .required(true),
                )
                .arg(
                    Arg::new("zkproof")
                        .long("zkproof")
                        .value_name("ZKPROOF")
                        .help("The ZK proof to verify")
                        .required(true),
                ),
        )
        .get_matches();

    // Match the subcommand and call the corresponding function
    match matches.subcommand() {
        Some(("upload", sub_matches)) => {
            let sbom_path = sub_matches.get_one::<String>("sbom").unwrap();
            upload_sbom(sbom_path);
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
