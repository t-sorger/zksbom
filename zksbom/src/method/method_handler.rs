use clap::error;
use log::{debug, info, warn, error};
use crate::method::merkle_tree::{create_commitment as create_merkle_commitment, MerkleRootLeaves, generate_proof};
use sp_core::{Hasher, H256};
use sp_runtime::traits::BlakeTwo256;
use binary_merkle_tree::{merkle_root, merkle_proof, verify_proof, MerkleProof};

use crate::database::db_commitment::{CommitmentDbEntry, init_db_commitment, insert_commitment, get_commitment as get_db_commitment, delete_db_commitment};

use crate::config::load_config;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

pub fn create_commitment(vulnerabilities: Vec<&str>) -> (String, Vec<String>) {
    // TODO: Implement handling for different methods
    let merkle_root_leaves = create_merkle_commitment(vulnerabilities);
    let commitment = merkle_root_leaves.root;
    let vulnerabilities = merkle_root_leaves.leaves;
    
    return (commitment, vulnerabilities);
}

pub fn get_commitment(vendor: &str, product: &str, version: &str) -> String {
    debug!("Getting commitment for vendor: {}, product: {}, version: {}", vendor, product, version);
    let commitment = get_db_commitment(vendor.to_string(), product.to_string(), version.to_string()).commitment;
    debug!("Commitment: {}", commitment);

    return commitment;
}

pub fn get_zkp(api_key: &str, method: &str, commitment: &str, vulnerability: &str) {
    error!("Implement get_zkp");
    
    match method{
        "Merkle Tree" => {
            info!("Merkle Tree");
            let proof = generate_proof(commitment.to_string(), vulnerability.to_string());
            
            print_proof(proof);
        }
        "zkp" => {
            info!("ZKP");
        }
        _ => {
            error!("Unknown method: {}", method);
        }
    }
}

fn print_proof(proof: MerkleProof<H256, H256>) {
    let config = load_config().unwrap(); // This will panic on error!
    let output_path = config.app.output;

    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        if let Err(e) = create_dir_all(parent) {
            eprintln!("Error creating directory: {}", e);
            return; // Exit early on error
        }
    }

    let mut file = match File::create(&output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return; // Exit early on error
        }
    };

    if let Err(e) = writeln!(file, "Root: {:?}", proof.root) {
        eprintln!("Error writing to file: {}", e);
        return;
    }
    if let Err(e) = writeln!(file, "Proof: {:?}", proof.proof) {
        eprintln!("Error writing to file: {}", e);
        return;
    }
    if let Err(e) = writeln!(file, "Number of Leaves: {:?}", proof.number_of_leaves) {
        eprintln!("Error writing to file: {}", e);
        return;
    }
    if let Err(e) = writeln!(file, "Leaf Index: {:?}", proof.leaf_index) {
        eprintln!("Error writing to file: {}", e);
        return;
    }
    if let Err(e) = writeln!(file, "Leaf: {:?}", proof.leaf) {
        eprintln!("Error writing to file: {}", e);
        return;
    }

    println!("Proof written to: {}", output_path);
}
