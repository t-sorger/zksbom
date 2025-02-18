// fn generate_proof(tree: &MerkleTree<Vec<u8>>, leaves: &Vec<Vec<u8>>, value: &str) -> Option<Proof<Vec<u8>>> {
//     let hashed_value = hash(value);
//     let index = leaves.iter().position(|leaf| *leaf == hashed_value)?;
//     Some(tree.proof(index))
// }
