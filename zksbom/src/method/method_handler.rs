use log::{debug, info, error};
use crate::method::merkle_tree::{create_commitment as create_merkle_commitment, MerkleRootLeaves};

pub fn create_commitment(vulnerabilities: Vec<&str>) -> String {
    // TODO: Implement handling for different methods
    let merkle_root_leaves = create_merkle_commitment(vulnerabilities);
    return merkle_root_leaves.root;
}

pub fn get_zkp(api_key: &str, method: &str, commitment: &str, vulnerability: &str) {
    panic!("Implement get_zkp");
    
    // if commitment.is_empty() {
    //     println!("Commitment is empty");
    //     return;
    // }
    // match mehtod{
    //     "merkle" => {
    //         println!("Merkle");
    //     }
    //     "zkp" => {
    //         println!("ZKP");
    //     }
    //     _ => {
    //         println!("Unknown method: {}", mehtod);
    //     }
    // }
}

