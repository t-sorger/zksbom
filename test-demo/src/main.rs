use sha2::{Sha256, Digest};
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use merkletree::hash::Algorithm;
use merkletree::proof::Proof;
use std::default::Default;

/// Custom SHA256 hash algorithm for Merkle tree
#[derive(Default, Clone)]
struct Sha256Algorithm(Sha256);

impl Algorithm<[u8; 32]> for Sha256Algorithm {
    fn hash(&mut self, data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Hash function to convert strings into SHA256 hashes
fn hash(data: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.finalize().into()
}

/// Create a Merkle tree from a list of vulnerabilities
fn create_merkle_tree(vulnerabilities: Vec<&str>) -> (MerkleTree<[u8; 32], Sha256Algorithm, VecStore<[u8; 32]>>, Vec<[u8; 32]>) {
    let leaves: Vec<[u8; 32]> = vulnerabilities.iter().map(|v| hash(v)).collect();
    let tree = MerkleTree::new(leaves.clone()).unwrap(); // Fixed method
    (tree, leaves)
}

/// Generate a proof for a given CVE if it exists in the Merkle tree
fn generate_proof(tree: &MerkleTree<[u8; 32], Sha256Algorithm, VecStore<[u8; 32]>>, leaves: &Vec<[u8; 32]>, value: &str) -> Option<Proof<[u8; 32]>> {
    let hashed_value = hash(value);
    let index = leaves.iter().position(|leaf| *leaf == hashed_value)?;
    Some(tree.gen_proof(index).unwrap())
}

/// Verify the proof against the stored Merkle root
fn verify_proof(tree: &MerkleTree<[u8; 32], Sha256Algorithm, VecStore<[u8; 32]>>, proof: &Proof<[u8; 32]>, value: &str) -> bool {
    let hashed_value = hash(value);
    proof.validate(tree.root(), &hashed_value)
}

fn main() {
    let vulnerabilities = vec![
        "CVE-123", "CVE-456", "CVE-789",
        "CVE-012", "CVE-345", "CVE-678",
        "CVE-901", "CVE-234", "CVE-567", "CVE-890"
    ];

    println!("📌 Step 1: Creating Merkle Tree...");
    let (tree, leaves) = create_merkle_tree(vulnerabilities.clone());
    let root_hash = tree.root();
    println!("✅ Merkle Tree Root: {:?}", hex::encode(root_hash));

    let cve_to_check = "CVE-123";
    println!("\n📌 Step 2: Generating Proof for {}", cve_to_check);
    if let Some(proof) = generate_proof(&tree, &leaves, cve_to_check) {
        println!("✅ Proof generated! Proof data: {:?}", proof);

        println!("\n📌 Step 3: Verifying Proof...");
        if verify_proof(&tree, &proof, cve_to_check) {
            println!("🎉 SUCCESS: {} is in the database!", cve_to_check);
        } else {
            println!("❌ ERROR: Proof verification failed.");
        }
    } else {
        println!("❌ ERROR: {} is NOT in the database!", cve_to_check);
    }

    let fake_cve = "CVE-abc";
    println!("\n📌 Step 4: Trying to generate proof for a non-existent CVE ({})", fake_cve);
    if let Some(proof) = generate_proof(&tree, &leaves, fake_cve) {
        println!("❌ Unexpected! Found proof for a non-existent CVE!");
        if verify_proof(&tree, &proof, fake_cve) {
            println!("❌ ERROR: Verification should have failed but passed.");
        } else {
            println!("✅ Verification correctly failed for fake CVE.");
        }
    } else {
        println!("✅ Correct behavior: {} is NOT in the database.", fake_cve);
    }
}
