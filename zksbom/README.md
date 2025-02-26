# zkSBOM

zkSBOM is a proof of concept (PoC) for disclosing limited but verifiable SBOM information to authorized users.

## Example Usage

### Uploading an SBOM as a Vendor

This command uploads the specified SBOM to the system.

```Bash
cargo run -- upload_sbom --api-key 123 --sbom ../sboms/zksbom-verifier.cdx.json
```

### Retrieving a Commitment

This command fetches the generated commitment for an uploaded SBOM, if available.

```Bash
cargo run -- get_commitment --vendor "Tom Sorger <sorger@kth.se>" --product "zksbom-verifier" --version "0.1.0"
```

### Obtaining the Zero-Knowledge Proof (ZKP)

There are two ways to retrieve the ZKP:

1. Provide the commitment of the product to be verified.
2. Provide the vendor name, product name, and product version for verification.

Additionally, the dependency to be checked must be specified.

#### Retrieving ZKP Using a Commitment

```Bash
cargo run -- get_zkp --api-key 123 --method "Merkle Tree" --commitment "0x0b9a83b952a61d281939e463e0848058e80271e4d2db5d294e4b7e8194276447" --dependency "binary-merkle-tree@16.0.0"
```

#### Retrieving ZKP Using Vendor, Product Name, and Version

```Bash
cargo run -- get_zkp_full --api-key 123 --method "Merkle Tree" --vendor "Tom Sorger <sorger@kth.se>" --product "zksbom-verifier" --version "0.1.0" --dependency "binary-merkle-tree@16.0.0"
```

### Possible Flags

- `--log_level`:
  - A string that specifies the log level.
  - Default: `"warn"`
- `--output`:
  - A string that specifies the path and filename for the output proof file.
  - Default: `"./tmp/output/proof.txt"`
- `--clean_init_dbs`:
  - A boolean that determines whether the databases should be deleted before running the application.
  - Default: `false`
- `--check_dependencies`:
  - A boolean that determines whether dependencies should be checked against [crates.io](https://crates.io/). This is only useful for Rust Project SBOMs.
  - Default: `false`
- `--check_dependencies_output`:
  - A string that specifies the path and filename for the output dependency check.
  - Default: `"./tmp/output/unfound_dependencies.log"`
- `--db_commitment_path`:
  - A string that specifies the path to the commitment database.
  - Default: `"./tmp/database/commitment.db"`
- `--db_sbom_path`:
  - A string that specifies the path to the SBOM database.
  - Default: `"./tmp/database/sbom.db"`
- `--db_dependency_path`:
  - A string that specifies the path to the dependency database.
  - Default: `"./tmp/database/dependency.db"`

If a flag is not specified, the default value will be used.

#### Example Usage with All Flags

Setting all configurations for this command is unnecessary.
Instead, it should provide an example demonstrating the use of all possible flags.

```Bash
cargo run -- upload_sbom --api-key 123 --sbom ../sboms/zksbom-verifier.cdx.json  --log_level "info" --output "./proof.txt" --clean_init_dbs true --check_dependencies true --check_dependencies_output "./unfound_dependencies.log" --db_commitment_path "./commitment.db" --db_sbom_path "./sbom.db" --db_dependency_path "./dependency.db"
```
