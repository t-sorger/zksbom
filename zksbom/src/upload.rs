use log::{debug, error};

use crate::database::db_sbom::{SbomDbEntry, insert_sbom};
use crate::database::db_commitment::{CommitmentDbEntry, insert_commitment};

use crate::method::method_handler::{create_commitment};

pub fn upload(api_key: &str, sbom_path: &str) {
    debug!("Uploading SBOM...");
    debug!("API Key: {}", api_key);
    debug!("SBOM Path: {}", sbom_path);

    // Step 1: Get the SBOM file content
    let sbom_content = match std::fs::read_to_string(&sbom_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read SBOM file: {}", e);
            return;
        }
    };

    debug!("SBOM Content: {}", sbom_content);
    
    // Step 2: Parse SBOM file for vulnerabilities, vendor, product, and version
    let vendor = "vendor"; // TODO: Parse vendor from SBOM
    let product = "product"; // TODO: Parse product from SBOM
    let version = "version"; // TODO: Parse version from SBOM

    let vulnerabilities = vec![
        "vuln1", "vuln2", "vuln3", "vuln4",
        "vuln5", "vuln6", "vuln7", "vuln8",
    ]; // TODO: Parse vulnerabilities from SBOM

    // Step 3: Save SBOM to database
    let sbom_entry = SbomDbEntry {
        vendor: vendor.to_string(),
        product: product.to_string(),
        version: version.to_string(),
        sbom: sbom_content.to_string(),
    };

    insert_sbom(sbom_entry);

    // Step 4: Generate Commitment
    let commitment = create_commitment(vulnerabilities);

    // Step 5: Save Commitment to database
    let commitment_entry = CommitmentDbEntry {
        vendor: vendor.to_string(),
        product: product.to_string(),
        version: version.to_string(),
        commitment: commitment.to_string(),
    };

    insert_commitment(commitment_entry);
}